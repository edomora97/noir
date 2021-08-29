(function() {var implementors = {};
implementors["rstream"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/structure/struct.DataType.html\" title=\"struct rstream::structure::DataType\">DataType</a>","synthetic":true,"types":["rstream::block::structure::DataType"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/structure/struct.BlockStructure.html\" title=\"struct rstream::structure::BlockStructure\">BlockStructure</a>","synthetic":true,"types":["rstream::block::structure::BlockStructure"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/structure/struct.OperatorStructure.html\" title=\"struct rstream::structure::OperatorStructure\">OperatorStructure</a>","synthetic":true,"types":["rstream::block::structure::OperatorStructure"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"rstream/structure/enum.OperatorKind.html\" title=\"enum rstream::structure::OperatorKind\">OperatorKind</a>","synthetic":true,"types":["rstream::block::structure::OperatorKind"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/structure/struct.OperatorReceiver.html\" title=\"struct rstream::structure::OperatorReceiver\">OperatorReceiver</a>","synthetic":true,"types":["rstream::block::structure::OperatorReceiver"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/structure/struct.Connection.html\" title=\"struct rstream::structure::Connection\">Connection</a>","synthetic":true,"types":["rstream::block::structure::Connection"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"rstream/structure/enum.ConnectionStrategy.html\" title=\"enum rstream::structure::ConnectionStrategy\">ConnectionStrategy</a>","synthetic":true,"types":["rstream::block::structure::ConnectionStrategy"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"rstream/enum.BatchMode.html\" title=\"enum rstream::BatchMode\">BatchMode</a>","synthetic":true,"types":["rstream::block::batcher::BatchMode"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/config/struct.EnvironmentConfig.html\" title=\"struct rstream::config::EnvironmentConfig\">EnvironmentConfig</a>","synthetic":true,"types":["rstream::config::EnvironmentConfig"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"rstream/config/enum.ExecutionRuntime.html\" title=\"enum rstream::config::ExecutionRuntime\">ExecutionRuntime</a>","synthetic":true,"types":["rstream::config::ExecutionRuntime"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/config/struct.LocalRuntimeConfig.html\" title=\"struct rstream::config::LocalRuntimeConfig\">LocalRuntimeConfig</a>","synthetic":true,"types":["rstream::config::LocalRuntimeConfig"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/config/struct.RemoteRuntimeConfig.html\" title=\"struct rstream::config::RemoteRuntimeConfig\">RemoteRuntimeConfig</a>","synthetic":true,"types":["rstream::config::RemoteRuntimeConfig"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/config/struct.RemoteHostConfig.html\" title=\"struct rstream::config::RemoteHostConfig\">RemoteHostConfig</a>","synthetic":true,"types":["rstream::config::RemoteHostConfig"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/config/struct.RemoteHostSSHConfig.html\" title=\"struct rstream::config::RemoteHostSSHConfig\">RemoteHostSSHConfig</a>","synthetic":true,"types":["rstream::config::RemoteHostSSHConfig"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.StreamEnvironment.html\" title=\"struct rstream::StreamEnvironment\">StreamEnvironment</a>","synthetic":true,"types":["rstream::environment::StreamEnvironment"]},{"text":"impl&lt;T&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.IterationStateHandle.html\" title=\"struct rstream::IterationStateHandle\">IterationStateHandle</a>&lt;T&gt;","synthetic":true,"types":["rstream::operator::iteration::IterationStateHandle"]},{"text":"impl&lt;Key, Out1, Out2, Keyer1, Keyer2, ShipStrat&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.JoinStreamLocalHash.html\" title=\"struct rstream::operator::join::JoinStreamLocalHash\">JoinStreamLocalHash</a>&lt;Key, Out1, Out2, Keyer1, Keyer2, ShipStrat&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShipStrat: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::join::local_hash::JoinStreamLocalHash"]},{"text":"impl&lt;Key, Out1, Out2, Keyer1, Keyer2, ShipStrat&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.JoinStreamLocalSortMerge.html\" title=\"struct rstream::operator::join::JoinStreamLocalSortMerge\">JoinStreamLocalSortMerge</a>&lt;Key, Out1, Out2, Keyer1, Keyer2, ShipStrat&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;ShipStrat: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::join::local_sort_merge::JoinStreamLocalSortMerge"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.ShipHash.html\" title=\"struct rstream::operator::join::ShipHash\">ShipHash</a>","synthetic":true,"types":["rstream::operator::join::ship::ShipHash"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.ShipBroadcastRight.html\" title=\"struct rstream::operator::join::ShipBroadcastRight\">ShipBroadcastRight</a>","synthetic":true,"types":["rstream::operator::join::ship::ShipBroadcastRight"]},{"text":"impl&lt;Key, Out1, Out2, Keyer1, Keyer2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.JoinStreamShipHash.html\" title=\"struct rstream::operator::join::JoinStreamShipHash\">JoinStreamShipHash</a>&lt;Key, Out1, Out2, Keyer1, Keyer2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::join::ship::JoinStreamShipHash"]},{"text":"impl&lt;Key, Out1, Out2, Keyer1, Keyer2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.JoinStreamShipBroadcastRight.html\" title=\"struct rstream::operator::join::JoinStreamShipBroadcastRight\">JoinStreamShipBroadcastRight</a>&lt;Key, Out1, Out2, Keyer1, Keyer2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::join::ship::JoinStreamShipBroadcastRight"]},{"text":"impl&lt;Key, Out1, Out2, OperatorChain1, OperatorChain2, Keyer1, Keyer2&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/join/struct.JoinStream.html\" title=\"struct rstream::operator::join::JoinStream\">JoinStream</a>&lt;Key, Out1, Out2, OperatorChain1, OperatorChain2, Keyer1, Keyer2&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Keyer2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;OperatorChain1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;OperatorChain2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out1: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out2: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::join::JoinStream"]},{"text":"impl&lt;Out&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/sink/struct.StreamOutput.html\" title=\"struct rstream::operator::sink::StreamOutput\">StreamOutput</a>&lt;Out&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::sink::StreamOutput"]},{"text":"impl&lt;Out&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/source/struct.CsvSource.html\" title=\"struct rstream::operator::source::CsvSource\">CsvSource</a>&lt;Out&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::source::csv::CsvSource"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/source/struct.FileSource.html\" title=\"struct rstream::operator::source::FileSource\">FileSource</a>","synthetic":true,"types":["rstream::operator::source::file::FileSource"]},{"text":"impl&lt;Out, It&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/source/struct.IteratorSource.html\" title=\"struct rstream::operator::source::IteratorSource\">IteratorSource</a>&lt;Out, It&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;It: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::source::iterator::IteratorSource"]},{"text":"impl&lt;Out, It, GenIt&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/source/struct.ParallelIteratorSource.html\" title=\"struct rstream::operator::source::ParallelIteratorSource\">ParallelIteratorSource</a>&lt;Out, It, GenIt&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;GenIt: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;It: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::source::parallel_iterator::ParallelIteratorSource"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.CountWindow.html\" title=\"struct rstream::operator::window::CountWindow\">CountWindow</a>","synthetic":true,"types":["rstream::operator::window::description::count_window::CountWindow"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.SessionEventTimeWindowDescr.html\" title=\"struct rstream::operator::window::SessionEventTimeWindowDescr\">SessionEventTimeWindowDescr</a>","synthetic":true,"types":["rstream::operator::window::description::session_window::SessionEventTimeWindowDescr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.SessionProcessingTimeWindowDescr.html\" title=\"struct rstream::operator::window::SessionProcessingTimeWindowDescr\">SessionProcessingTimeWindowDescr</a>","synthetic":true,"types":["rstream::operator::window::description::session_window::SessionProcessingTimeWindowDescr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.SlidingEventTimeWindowDescr.html\" title=\"struct rstream::operator::window::SlidingEventTimeWindowDescr\">SlidingEventTimeWindowDescr</a>","synthetic":true,"types":["rstream::operator::window::description::sliding_window::SlidingEventTimeWindowDescr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.SlidingProcessingTimeWindowDescr.html\" title=\"struct rstream::operator::window::SlidingProcessingTimeWindowDescr\">SlidingProcessingTimeWindowDescr</a>","synthetic":true,"types":["rstream::operator::window::description::sliding_window::SlidingProcessingTimeWindowDescr"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.ProcessingTimeWindow.html\" title=\"struct rstream::operator::window::ProcessingTimeWindow\">ProcessingTimeWindow</a>","synthetic":true,"types":["rstream::operator::window::description::ProcessingTimeWindow"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.EventTimeWindow.html\" title=\"struct rstream::operator::window::EventTimeWindow\">EventTimeWindow</a>","synthetic":true,"types":["rstream::operator::window::description::EventTimeWindow"]},{"text":"impl&lt;'a, Key, Out&gt; !<a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/operator/window/struct.Window.html\" title=\"struct rstream::operator::window::Window\">Window</a>&lt;'a, Key, Out&gt;","synthetic":true,"types":["rstream::operator::window::Window"]},{"text":"impl&lt;Out&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"enum\" href=\"rstream/operator/enum.StreamElement.html\" title=\"enum rstream::operator::StreamElement\">StreamElement</a>&lt;Out&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::operator::StreamElement"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.ExecutionMetadata.html\" title=\"struct rstream::ExecutionMetadata\">ExecutionMetadata</a>","synthetic":true,"types":["rstream::scheduler::ExecutionMetadata"]},{"text":"impl&lt;Out, OperatorChain&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.Stream.html\" title=\"struct rstream::Stream\">Stream</a>&lt;Out, OperatorChain&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;OperatorChain: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::stream::Stream"]},{"text":"impl&lt;Key, Out, OperatorChain&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.KeyedStream.html\" title=\"struct rstream::KeyedStream\">KeyedStream</a>&lt;Key, Out, OperatorChain&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;OperatorChain: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::stream::KeyedStream"]},{"text":"impl&lt;Out, OperatorChain, WinOut, WinDescr&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.WindowedStream.html\" title=\"struct rstream::WindowedStream\">WindowedStream</a>&lt;Out, OperatorChain, WinOut, WinDescr&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;OperatorChain: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;WinDescr: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;WinOut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::stream::WindowedStream"]},{"text":"impl&lt;Key, Out, OperatorChain, WinOut, WinDescr&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a> for <a class=\"struct\" href=\"rstream/struct.KeyedWindowedStream.html\" title=\"struct rstream::KeyedWindowedStream\">KeyedWindowedStream</a>&lt;Key, Out, OperatorChain, WinOut, WinDescr&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Key: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;OperatorChain: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Out: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;WinDescr: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;WinOut: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.54.0/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,&nbsp;</span>","synthetic":true,"types":["rstream::stream::KeyedWindowedStream"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()