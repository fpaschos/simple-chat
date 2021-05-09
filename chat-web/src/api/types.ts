export type ID = string;
export type UserId = string;

export enum ClientMessageType {
    SendMessage = 'SendMessage',
};

export enum ServerMessageType {
    InvalidCommand = 'InvalidCommand',
    ChatMessage = 'ChatMessage',
};

// Output messages
export type SendMessage = {
    type: ClientMessageType.SendMessage,
    user: UserId,
    content: string,
};

// Input messages
export type InvalidCommand = {
    type: ServerMessageType.InvalidCommand,
};

export interface ChatMessage extends Message {
    type: ServerMessageType.ChatMessage,
}


export type Message = {
    id: ID,
    channel_id: ID,
    sender: UserId,
    created: number,
    content: string,
};