import { createSlice, PayloadAction } from "@reduxjs/toolkit";

const initialState: number[] = [];

export const runningBotsSlice = createSlice({
    name: "runningBots",
    initialState,
    reducers: {
        setRunningBots: (_, action: PayloadAction<number[]>) => {
            return action.payload;
        }
    }
});

export const { setRunningBots } = runningBotsSlice.actions;