import {useMyWebSocket} from "../App";
import {useState} from "react";

function AddPrinterWidget() {
    const {sendJsonMessage} = useMyWebSocket();
    const [printerName, setPrinterName] = useState("")
    const [printerIP, setPrinterIP] = useState("")

    return (
        <div className={"printer_widget"}>
            <h1 title={"Add new printer"}>Add new printer</h1>
            <form onSubmit={
                e => {
                    sendJsonMessage({action: "add", ip_address: printerIP, name: printerName});
                    setPrinterName("");
                    setPrinterIP("");
                    e.preventDefault();
                }

            }>
                <div style={{textAlign: "left", margin: "1em 0"}}>
                    <label>
                        Printer Name
                        <input
                            value={printerName}
                            required={true}
                            style={{float: "right"}}
                            name={"Printer Name"}
                            onChange={e => setPrinterName(e.target.value)}
                        />
                    </label>
                </div>
                <div style={{textAlign: "left", margin: "1em 0"}}>
                    <label>
                        IP Address
                        <input
                            value={printerIP}
                            required={true}
                            style={{float: "right"}}
                            name={"IP Address"}
                            type="text" minLength="7"
                            maxLength="15"
                            size="15"
                            pattern="^((\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.){3}(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])$"
                            onChange={e => setPrinterIP(e.target.value)}
                        />
                    </label>
                    <button
                        style={{margin: "0 .5em", position: "absolute", bottom: ".5em", right: ".5em", left: ".5em"}}
                        disabled={!printerName || !printerIP}
                    >
                        Add Printer
                    </button>
                </div>
            </form>
        </div>
    )
}

export default AddPrinterWidget;
