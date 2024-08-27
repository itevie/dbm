import { configureStore } from "@reduxjs/toolkit";
import { TypedUseSelectorHook, useSelector } from 'react-redux';
import { settingsSlice } from "./stores/options";
import { botsSlice } from "./stores/bots";
import { Root } from "react-dom/client";
import { commandsSlice } from "./stores/commands";
import { runningBotsSlice } from "./stores/runningBots";

const store = configureStore({
    reducer: {
        settings: settingsSlice.reducer,
        bots: botsSlice.reducer,
        commands: commandsSlice.reducer,
        runningBots: runningBotsSlice.reducer
    }
});

export type RootState = ReturnType<typeof store.getState>;
export const useSettingsSelector: TypedUseSelectorHook<RootState> = useSelector;
export const useBotsSelector: TypedUseSelectorHook<RootState> = useSelector;
export const useMakerSelector: TypedUseSelectorHook<RootState> = useSelector;
export default store;
