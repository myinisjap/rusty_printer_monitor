import ProgressBar from "./progress_bar"
import {useState} from "react";

function sendMessage(message) {
  console.log(message + " printer")
}

function startPrint (file) {
  console.log(file)
}

function deleteWidget (name) {
    console.log("Deleting widget" + name)
}

function PrinterWidget(props) {
  const [fileDropDown, setFileDropDown] = useState()
  const handleChange = (e) => {
    setFileDropDown(e.target.value );
  }
  return (
    <div style={{
      border: "solid",
      borderRadius: "1em",
      width: "20em",
      minWidth: "20em",
      minHeight: "26em",
      margin: ".5em",
      padding: ".5em",
      resize: "both",
      overflow: "auto",
      position: "relative",
      background: "cornsilk"
    }}>
      <h1 style={{display: "inline"}} title={props.printer_name}>{props.printer_name}</h1>
      <button style={{float: "right"}} onClick={() => deleteWidget(props.printer_name)}>X</button>
        <p><strong>IP Address:</strong> {props.ip_address}</p>
        <div>
          <button style={{margin: "0 .5em"}} onClick={() => sendMessage("pause")}>Pause Printer</button>
          <button style={{margin: "0 .5em"}} onClick={() => sendMessage("stop")}>Stop Printer</button>
        </div>
        <div style={{inset: ".5em", width: "100%", height: "calc(100% - 11em"}}>
          <h3>Files available on Printer</h3>
          <select size={10} style={{width: "100%", height: "calc(100% - 4em)", overflow: "scroll", }} onChange={handleChange}>
            {props.files_available.map((file) =>
              <option key={file}>{file}</option>
            )}
          </select>
        </div>
        
        <div style={{bottom: ".5em", left:".5em", right: ".5em", position: "absolute",}}>
          <ProgressBar progress={props.progress}/>
          <button
            style={{margin: "0 .5em"}}
            onClick={() => startPrint(fileDropDown)}>
            Start Print
          </button>
        </div>
      {/* </div> */}
    </div>
  )
}

export default PrinterWidget;
