const detailsContent = $("#details-content");

const processData = (structures, profilers) => {
    resetGraph();
    const profiler = new Profiler(profilers);
    const [nodes, links] = buildJobGraph(structures, profiler);
    drawNetwork(nodes, links);
};

const formatBytes = (bytes) => {
    const fmt = d3.format('.1f');
    if (bytes < 1024) return `${fmt(bytes)}B`;
    if (bytes < 1024*1024) return `${fmt(bytes / 1024)}KiB`;
    if (bytes < 1024*1024*1024) return `${fmt(bytes / 1024 / 1024)}MiB`;
    return `${fmt(bytes / 1024 / 1024 / 1024)}GiB`;
};

const formatNumber = (num) => {
    return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

const drawOperatorDetails = (block_id, operator, replicas, linkMetrics) => {
    detailsContent.html("");
    detailsContent.append(
        $("<p>")
            .append($("<strong>").text("Operator: "))
            .append($("<code>").text(operator.title))
    );
    if (operator.subtitle) {
        detailsContent.append(
            $("<p>")
                .append($("<span>").text(operator.subtitle))
        );
    }

    const hostCounts = {};
    for (const {host_id} of replicas) {
        if (!(host_id in hostCounts)) hostCounts[host_id] = 0;
        hostCounts[host_id] += 1;
    }
    detailsContent.append($("<p>").append($("<strong>").text("Replicated at:")));
    const replicasList = $("<ul>");
    for (const [host_id, count] of Object.entries(hostCounts)) {
        replicasList.append(
            $("<li>")
                .append($("<code>").text(`Host${host_id}`))
                .append(` × ${count}`));
    }
    detailsContent.append(replicasList);

    detailsContent.append(
        $("<p>")
            .append($("<strong>").text("Produces: "))
            .append($("<code>").text(operator.out_type))
    );
    if (operator.connections.length > 0) {
        const list = $("<ul>");
        for (const connection of operator.connections) {
            const to_block_id = connection.to_block_id;
            const li = $("<li>")
                .append("Block " + to_block_id + " sending ")
                .append($("<code>").text(connection.data_type))
                .append(" with strategy ")
                .append($("<code>").text(connection.strategy));
            const key = ChannelMetric.blockPairKey(block_id, connection.to_block_id);
            if (key in linkMetrics.items_out) {
                const drawMessages = () => {
                    drawGraph(linkMetrics.items_out[key].series, `Items/s in ${block_id} → ${to_block_id}`, (v) => formatNumber(v));
                };
                const drawNetworkMessages = () => {
                    drawGraph(linkMetrics.net_messages_out[key].series, `Network messages/s in ${block_id} → ${to_block_id}`);
                };
                const drawNetworkBytes = () => {
                    drawGraph(linkMetrics.net_bytes_out[key].series, `Network bytes/s in ${block_id} → ${to_block_id}`, (v) => formatBytes(v)+"/s");
                };
                const total = linkMetrics.items_out[key].series.total;
                li.append(": ")
                    .append($("<a>").attr("href", "#").on("click", () => drawMessages()).text(`${formatNumber(total)} items sent`));

                drawMessages();
                if (key in linkMetrics.net_messages_out) {
                    const numMex = linkMetrics.net_messages_out[key].series.total;
                    const bytes = linkMetrics.net_bytes_out[key].series.total;
                    li
                        .append(" (in ")
                        .append($("<a>").attr("href", "#").on("click", () => drawNetworkMessages()).text(`${numMex} messages`))
                        .append(", for a ")
                        .append($("<a>").attr("href", "#").on("click", () => drawNetworkBytes()).text(`total of ${formatBytes(bytes)}`))
                        .append(")");

                }
            }
            list.append(li);
        }
        detailsContent.append($("<p>")
            .append($("<strong>").text("Connects to: "))
            .append(list));
    }
    if (operator.receivers.length > 0) {
        const list = $("<ul>");
        for (const receiver of operator.receivers) {
            const from_block_id = receiver.previous_block_id;
            const li = $("<li>")
                .append("Block " + from_block_id + " receiving ")
                .append($("<code>").text(receiver.data_type));
            const key = ChannelMetric.blockPairKey(receiver.previous_block_id, block_id);
            if (key in linkMetrics.items_in) {
                const drawMessages = () => {
                    drawGraph(linkMetrics.items_in[key].series, `Items/s in ${from_block_id} → ${block_id}`, (v) => formatNumber(v));
                };
                const drawNetworkMessages = () => {
                    drawGraph(linkMetrics.net_messages_in[key].series, `Network messages/s in ${from_block_id} → ${block_id}`);
                };
                const drawNetworkBytes = () => {
                    drawGraph(linkMetrics.net_bytes_in[key].series, `Network bytes/s in ${from_block_id} → ${block_id}`, (v) => formatBytes(v)+"/s");
                };
                const total = linkMetrics.items_in[key].series.total;
                li.append(": ")
                    .append($("<a>").attr("href", "#").on("click", () => drawMessages()).text(`${formatNumber(total)} items received`));

                drawMessages();
                if (key in linkMetrics.net_messages_in) {
                    const numMex = linkMetrics.net_messages_in[key].series.total;
                    const bytes = linkMetrics.net_bytes_in[key].series.total;
                    li
                        .append(" (in ")
                        .append($("<a>").attr("href", "#").on("click", () => drawNetworkMessages()).text(`${numMex} messages`))
                        .append(", for a ")
                        .append($("<a>").attr("href", "#").on("click", () => drawNetworkBytes()).text(`total of ${formatBytes(bytes)}`))
                        .append(")");

                }
            }
            list.append(li);
        }
        detailsContent.append($("<p>")
            .append($("<strong>").text("Receives data from: "))
            .append(list));
    }
};

const drawLinkDetails = (from_block_id, connection, linkMetrics) => {
    const to_block_id = connection.to_block_id;
    detailsContent.html("");
    detailsContent.append(
        $("<p>")
            .append($("<strong>").text("Connection: "))
            .append($("<code>").text(`${from_block_id} → ${to_block_id}`))
    );
    detailsContent.append(
        $("<p>")
            .append($("<strong>").text("Data type: "))
            .append($("<code>").text(connection.data_type))
    );
    detailsContent.append(
        $("<p>")
            .append($("<strong>").text("Strategy: "))
            .append($("<code>").text(connection.strategy))
    );

    const metricsKey = ChannelMetric.blockPairKey(from_block_id, to_block_id);
    if (metricsKey in linkMetrics.net_messages_in) {
        const message = linkMetrics.net_messages_in[metricsKey].series.total;
        const bytes = linkMetrics.net_bytes_in[metricsKey].series.total;
        const drawNetworkMessages = () => {
            drawGraph(linkMetrics.net_messages_in[metricsKey].series, `Network messages/s in ${from_block_id} → ${to_block_id}`);
        };
        const drawNetworkBytes = () => {
            drawGraph(linkMetrics.net_bytes_in[metricsKey].series, `Network bytes/s in ${from_block_id} → ${to_block_id}`, (v) => formatBytes(v)+"/s");
        };
        detailsContent.append($("<p>")
            .append($("<strong>").text("Traffic: "))
            .append($("<a>").attr("href", "#").on("click", () => drawNetworkMessages()).text(`${message} messages`))
            .append(", for a ")
            .append($("<a>").attr("href", "#").on("click", () => drawNetworkBytes()).text(`total of ${formatBytes(bytes)}`))
            .append(` (${formatBytes(bytes/message)}/message)`));
        drawNetworkBytes();
    }
}

const buildJobGraph = (structures, profiler) => {
    const linkMetrics = {
        items_in: profiler.channel_metrics.items_in.groupByBlockId(),
        items_out: profiler.channel_metrics.items_out.groupByBlockId(),
        net_messages_in: profiler.channel_metrics.net_messages_in.groupByBlockId(),
        net_messages_out: profiler.channel_metrics.net_messages_out.groupByBlockId(),
        net_bytes_in: profiler.channel_metrics.net_bytes_in.groupByBlockId(),
        net_bytes_out: profiler.channel_metrics.net_bytes_out.groupByBlockId(),
    };

    const byBlockId = {};
    const blockReplicas = {};
    for (const entry of structures) {
        const [coord, structure] = entry;
        const block_id = coord["block_id"];
        byBlockId[block_id] = structure;
        if (!(block_id in blockReplicas)) blockReplicas[block_id] = [];
        blockReplicas[block_id].push(coord);
    }
    const nodes = [];
    const links = [];
    const receivers = {};

    const operatorId = (block_id, index) => {
        return 100000 + block_id * 1000 + index;
    };

    const maxChannelBytes = Math.max(...Object.values(linkMetrics.net_bytes_in).map((d) => d.series.total));
    const linkWidth = (from_block_id, to_block_id) => {
        const key = ChannelMetric.blockPairKey(from_block_id, to_block_id);
        const minWidth = 1;
        const maxWidth = 3;
        const metric = linkMetrics.net_bytes_in[key];
        if (!metric) return minWidth;
        const value = metric.series.total;
        return minWidth + (maxWidth - minWidth) * (value / maxChannelBytes);
    }

    for (const [block_id, structure] of Object.entries(byBlockId)) {
        const block = {
            id: block_id,
            data: {
                text: "Block " + block_id
            },
            children: structure.operators.map((operator, index) => {
                return {
                    id: operatorId(block_id, index),
                    data: {
                        text: operator["title"],
                        onclick: () => drawOperatorDetails(block_id, operator, blockReplicas[block_id], linkMetrics)
                    }
                };
            })
        };
        nodes.push(block);
        structure.operators.map((operator, index) => {
            if (index < structure.operators.length - 1) {
                links.push({
                    source: operatorId(block_id, index),
                    target: operatorId(block_id, index+1),
                    data: {
                        text: operator.out_type,
                    }
                })
            }
            for (const receiver of operator.receivers) {
                const prev_block_id = receiver.previous_block_id;
                if (!(prev_block_id in receivers)) receivers[prev_block_id] = {};
                receivers[prev_block_id][block_id] = index;
            }
        });
    }
    for (const [block_id, structure] of Object.entries(byBlockId)) {
        structure.operators.map((operator, index) => {
            for (const connection of operator.connections) {
                const receiverIndex = receivers[block_id][connection.to_block_id];
                const source = operatorId(block_id, index);
                const target = operatorId(connection.to_block_id, receiverIndex);
                links.push({
                    source,
                    target,
                    data: {
                        type: "solid",
                        text: connection.data_type,
                        onclick: () => drawLinkDetails(block_id, connection, linkMetrics),
                        width: linkWidth(block_id, connection.to_block_id),
                    }
                })
            }
        });
    }

    return [nodes, links];
};