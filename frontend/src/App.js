import './App.css';
import PrinterWidget from "./widget/printer";
import { useEffect, useState } from "react";
import useWebSocket from 'react-use-websocket';


export const App = () => {
  const [printers, setPrinters] = useState(
    [
      // {"printer_name": "a",
      // "ip_address": "192.168.1.1",
      // "files_available": [
      //     "ibsedfjinwojfnkoweokfkpowef",
      //     "wnejifnwioefnmkowmekfmwpokemfpwmepfoiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii",
      //  ]
      // "progress": 50},
    ]
  );
// eslint-disable-next-line
  const WS_URL = ((window.location.protocol == "https:" && "wss://") || "ws://") + window.location.host + "/ws";
// eslint-disable-next-line
  const { sendJsonMessage, lastJsonMessage } = useWebSocket(
    WS_URL,
    {
      share: true,
      shouldReconnect: () => true,
    },
  )
  useEffect(() => {
    if (lastJsonMessage !== null) {
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
      {printers.map((i) =>
        < PrinterWidget key={i.printer_name} {...i} />
      )}
    </div>
  );
}

export default App;
