import ProgressBar from "./progress_bar"
import {useState} from "react";
import {useMyWebSocket} from "../App";
import { isWindows } from "react-device-detect"

function PrinterWidget(props) {
    const {sendJsonMessage} = useMyWebSocket();
    const [fileDropDown, setFileDropDown] = useState()
    const handleChange = (e) => {
        setFileDropDown(e.target.value);
    }
    let fileWindowSubtract = isWindows ? "19em" : "11em";

    return (
        <div className={"printer_widget"}>
            <h1 style={{display: "inline"}} title={props.printer_name}>{props.printer_name}</h1>
            <button style={{float: "right"}} onClick={() =>
                sendJsonMessage({action: "remove", name: props.printer_name})}>X
            </button>
            <p><strong>IP Address:</strong> {props.ip_address}</p>
            <div>
                <button style={{margin: "0 .5em"}} onClick={() =>
                    sendJsonMessage({action: "pause", ip_address: props.ip_address})}>Pause
                    Printer
                </button>
                <button style={{margin: "0 .5em"}} onClick={() =>
                    sendJsonMessage({action: "stop", ip_address: props.ip_address})}>Stop Printer
                </button>
            </div>
            <div style={{inset: ".5em", width: "100%", height: `calc(100% - ${fileWindowSubtract}`}}>
                <h3>Files available on Printer</h3>
                <select size={10} style={{width: "100%", height: "calc(100% - 4em)", overflow: "scroll",}}
                        onChange={handleChange}>
                    {props.files_available.map((file) =>
                        <option key={file}>{file}</option>
                    )}
                </select>
            </div>

            <div style={{bottom: ".5em", left: ".5em", right: ".5em", position: "absolute",}}>
                <ProgressBar progress={props.progress}/>
                <button
                    style={{margin: "0 .5em"}}
                    disabled={!fileDropDown}
                    onClick={() =>
                        sendJsonMessage({action: "start", ip_address: props.ip_address, file: fileDropDown})
                }>
                    Start Print
                </button>
            </div>
        </div>
    )
}

export default PrinterWidget;
