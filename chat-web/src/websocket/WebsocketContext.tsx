import React, { createContext, PropsWithChildren, useEffect, useRef } from "react";
import { useDispatch } from "react-redux";
import { ChatMessage, ServerMessageType } from "../api/types";
import { channelMessage } from "../channel/module";


interface WSContextType {
    sendMessage: (msg: any) => void,
};

export const WebSocketContext = createContext<WSContextType | undefined>(undefined);


const WebSocketProvider: React.FC<PropsWithChildren<{}>> = ({ children }) => {
    const dispatch = useDispatch();
    const socket = useRef<WebSocket | null>(null)

    const sendMessage = (msg: any) => {
        if (socket.current) {
            socket.current.send(JSON.stringify(msg))
        }
    };

    let ws = {
        sendMessage,
    }

    // Use an effect to initialize the websocket connection once
    useEffect(() => {
        socket.current = new WebSocket("ws://localhost:9090/chat");

        socket.current.onclose = () => {
            console.log("WS closed")
        };

        socket.current.onerror = (error) => {
            let err = error as ErrorEvent
            console.log(`WS error: ${err.message}`)
        }

        socket.current.onmessage = (evt: MessageEvent) => {
            console.log(`WS RAW msg: ${evt.data}`)
            let payload
            try {
                payload = JSON.parse(evt.data)
            } catch {
                console.error(`WS Invalid JSON message: ${evt.data} `)
                return
            }

            if (payload.type === ServerMessageType.ChatMessage) {
                let msg = payload as ChatMessage
                dispatch(channelMessage(msg))
            } else {
                console.error(`WS Invalid message: ${payload} `)
            }
        }

        return () => {
            if (socket.current)
                socket.current.close()
        }
    }, [dispatch])  // Note executes only once per render

    return (
        <WebSocketContext.Provider value={ws}>
            {children}
        </WebSocketContext.Provider>
    );
};

export default WebSocketProvider;