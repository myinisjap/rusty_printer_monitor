import './App.css';
import PrinterWidget from "./widget/printer";
import React, {useEffect, useState} from "react";
import useWebSocket from 'react-use-websocket';


// eslint-disable-next-line
export const useMyWebSocket = () => {
    return useWebSocket(
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
        const { lastJsonMessage } = useMyWebSocket();
        useEffect(() => {
            if (lastJsonMessage !== null && Array.isArray(lastJsonMessage)) {
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
                {printers.map((i) =>
                    < PrinterWidget key={i.printer_name} {...i} />
                )}
            </div>
        );
    }

    export default App;
