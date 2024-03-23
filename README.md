# Log Loader

Log Loader is a command-line program designed to load log files, parse them, and upload the data to ElasticSearch. This tool simplifies the process of handling log data for analysis and storage.

## Usage

To use Log Loader, follow the command structure below:

```bash
log-loader [OPTIONS] --file-name <FILE_NAME>
```
## Example:
```bash
./launcher --file-names test_log.txt
```
## Output:
```bash
              _
            /~_)                                                      /'
        ~-/'-~                                                      /'
        /'      ____     ____         ____     ,____     ____     /'          ____     O  ____
      /'      /'    )--/'    )      /'    )   /'    )  /'    )  /' /'    /  /'    )--/' /'    )--
 /~\,'   _  /'    /' /'    /'     /'    /'  /'    /' /'    /' /' /'    /'  '---,   /'  '---,
(,/'`\____)(___,/'  (___,/(__    (___,/(__/'    /(__(___,/(__(__(___,/(__(___,/   (__(___,/
                       /'                                          /'
               /     /'                                    /     /'
              (___,/'                                     (___,/'
                                    Author: Fei Wang
                                    Version: v1.0.0
2024-03-23T16:03:47.766465Z  INFO ThreadId(10) launcher/src/main.rs:53: Checking the log file names and the path of the file...
2024-03-23T16:03:47.766509Z  INFO ThreadId(10) launcher/src/main.rs:55: log file names: ["test_log.txt"]
2024-03-23T16:03:47.766525Z  INFO ThreadId(10) launcher/src/main.rs:56: Path of the file: "log_file"
2024-03-23T16:03:47.766552Z  INFO ThreadId(10) launcher/src/main.rs:59: Current directory is: "/Users/michaljason/Documents/log-parser"
2024-03-23T16:03:47.766576Z  INFO ThreadId(10) launcher/src/main.rs:65: File path is: "/Users/michaljason/Documents/log-parser/log_file/test_log.txt"
2024-03-23T16:03:47.766611Z  INFO ThreadId(10) launcher/src/main.rs:73: File "/Users/michaljason/Documents/log-parser/log_file/test_log.txt" exist
2024-03-23T16:03:47.767371Z  INFO ThreadId(01) launcher/src/main.rs:27: File vector: ["/Users/michaljason/Documents/log-parser/log_file/test_log.txt"]
2024-03-23T16:03:47.767479Z  INFO ThreadId(08) loader/src/data_loder.rs:20: trying to read the file: "/Users/michaljason/Documents/log-parser/log_file/test_log.txt"
2024-03-23T16:03:47.767594Z  INFO ThreadId(08) loader/src/data_loder.rs:22: file opened successfully, file path: "/Users/michaljason/Documents/log-parser/log_file/test_log.txt"
2024-03-23T16:03:47.767617Z  INFO ThreadId(08) loader/src/data_loder.rs:25: reader created successfully, file path: "/Users/michaljason/Documents/log-parser/log_file/test_log.txt"
2024-03-23T16:03:47.792288Z  INFO ThreadId(08) loader/src/data_loder.rs:56: Log item is 2014/Oct/24 19:16:44.288749 mtc PORTEVENT WCG10014_0624.ttcn:34(testcase:WCG100140624) Port dnsInternalPort[0] was started.----------------Data frame is Some(DataFrame { time: "2014/Oct/24 19:16:44.288749", component: "mtc", peer_component: "WCG10014_0624", event_type: "PORTEVENT", event_description: "ttcn", port: 34, function_name: "testcase:WCG100140624", message: " Port dnsInternalPort[0] was started" })
2024-03-23T16:03:47.798083Z  INFO ThreadId(08) loader/src/data_loder.rs:56: Log item is 2014/Oct/24 19:16:44.288792 mtc PORTEVENT WCG10014_0624.ttcn:34(testcase:WCG100140624) Port dnsInternalPort[1] was started.----------------Data frame is Some(DataFrame { time: "2014/Oct/24 19:16:44.288792", component: "mtc", peer_component: "WCG10014_0624", event_type: "PORTEVENT", event_description: "ttcn", port: 34, function_name: "testcase:WCG100140624", message: " Port dnsInternalPort[1] was started" })
2024-03-23T16:03:47.803418Z  INFO ThreadId(08) loader/src/data_loder.rs:56: Log item is 2014/Oct/24 18:34:38.950664 mtc TIMEROP MsrpLayerComponent.ttcn:213(function:receiveInternalMessage) Start timer TimerTestCaseInternalCommunicationGuard: 185 s----------------Data frame is Some(DataFrame { time: "2014/Oct/24 18:34:38.950664", component: "mtc", peer_component: "MsrpLayerComponent", event_type: "TIMEROP", event_description: "ttcn", port: 213, function_name: "function:receiveInternalMessage", message: " Start timer TimerTestCaseInternalCommunicationGuard: 185 s" })
2024-03-23T16:03:47.829393Z  INFO ThreadId(08) loader/src/data_loder.rs:37: Log item is 2014/Oct/24 18:34:38.950563 mtc PORTEVENT MsrpLayerComponent.ttcn:168(function:sendInternalMessage) Sent on msrpInternalPort[0] to 850 @variables.internalPortMessageWithMsrpMessages : {    internalMessage := {        description := "COMMAND_COMPONENT_STOP",        parameters := { }    },    msrpMessages := { }----------------Data frame is Some(DataFrame { time: "2014/Oct/24 18:34:38.950563", component: "mtc", peer_component: "MsrpLayerComponent", event_type: "PORTEVENT", event_description: "ttcn", port: 168, function_name: "function:sendInternalMessage", message: " Sent on msrpInternalPort[0] to 850 @variables" })

```


