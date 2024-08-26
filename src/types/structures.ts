export interface Bot {
    id: number,
    name: string,
    description: string,
    token: string,
}

export interface Settings {
    current_bot: number | null,
}

export interface Command {
    id: number,
    name: string,
    bot_id: number,
    code_id: number | null,
}

export interface CodePiece {
    id: number,
    code: string,
}