initSidebarItems({"struct":[["KeyedStream","A `KeyedStream` is like a set of `Stream`s, each of which partitioned by some `Key`. Internally it's just a stream whose elements are `KeyValue` pairs and the operators behave following the `KeyedStream` semantics."],["KeyedWindowedStream","A `KeyedWindowedStream` is a data stream partitioned by `Key`, where elements of each partition are divided in groups called windows. Windows are handled independently for each partition of the stream."],["Stream","A Stream represents a chain of operators that work on a flow of data. The type of the elements entering the stream is `In`, while the type of the outgoing elements is `Out`."],["WindowedStream","A `WindowedStream` is a data stream where elements are divided in multiple groups called windows. Internally, a `WindowedStream` is just a `KeyedWindowedStream` where each element is assigned to the same key `()`."]],"type":[["BlockId","Identifier of a block in the job graph."],["KeyValue","On keyed streams, this is the type of the items of the stream."]]});