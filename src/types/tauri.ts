export interface TauriEvents {
    "running_bots_update": {
        list: number[]
    },
    "error": {
        error: {
            message: string,
            error_type: string,
            source: string | null,
            location: {
                start: number,
                end: number,
                line: number,
                context: string
            } | null,
        }
    }
}