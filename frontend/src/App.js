import './App.css';
import PrinterWidget from "./widget/printer";
import React, {useEffect, useState} from "react";
import useWebSocket from 'react-use-websocket';
import AddPrinterWidget from "./widget/add_printer";



export const useMyWebSocket = () => {
    return useWebSocket(
        // eslint-disable-next-line
        ((window.location.protocol == "https:" && "wss://") || "ws://") + window.location.host + "/ws",
        // "ws://0.0.0.0:8000/ws",
        {
            share: true,
            shouldReconnect: () => true,
        },
    )
}

export const App = () => {
    const [printers, setPrinters] = useState(
        [
            // {"printer_name": "a",
            // "ip_address": "192.168.1.1",
            // "files_available": [
            //     "ibsedfjinwojfnkoweokfkpowef",
            //     "wnejifnwioefnmkowmekfmwpokemfpwmepfoiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
            //  ],
            // "progress": 50},
        ]
    );
    const {lastJsonMessage} = useMyWebSocket();
    useEffect(() => {
        if (lastJsonMessage !== null && Array.isArray(lastJsonMessage)) {
            console.log(lastJsonMessage);
            setPrinters(lastJsonMessage)
        }
    }, [lastJsonMessage, setPrinters])

    return (
        <div style={{
            textAlign: "center",
            display: "flex",
            flexDirection: "row",
            flexWrap: "wrap",
        }}>
            {
                printers.map((i) =>
                    < PrinterWidget key={i.printer_name} {...i} />
                )
            }
            < AddPrinterWidget />
        </div>
    );
}

export default App;
