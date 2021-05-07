export type ID = string;
export type Message = string;

export enum ClientMessageType {
    Join = 'Join',
    SendMessage = 'SendMessage',
};

export enum ServerMessageType {
    InvalidCommand = 'InvalidCommand',
    ChatMessage = 'ChatMessage',
};

export type Join = {
    type: ClientMessageType.Join,
    user: ID,
};

// Output messages
export type SendMessage = {
    type: ClientMessageType.SendMessage,
    user: ID,
    msg: Message,
};

// Input messages
export type InvalidCommand = {
    type: ServerMessageType.InvalidCommand,
};

export type ChatMessage = {
    type: ServerMessageType.ChatMessage,
    user: ID,
    msg: Message,
}