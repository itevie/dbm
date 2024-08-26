import React, { useRef } from "react";
import HeaderText from "../components/HeaderText";
import { invoke } from "@tauri-apps/api";
import { setSettings } from "../stores/options";
import { useDispatch } from "react-redux";
import { addBot } from "../stores/bots";
import { Bot } from "../types/structures";

export default function CreateBot() {
    const dispatch = useDispatch();
    const nameRef = useRef<HTMLInputElement>(null);
    const tokenRef = useRef<HTMLInputElement>(null);


    async function createBot() {
        let result = await invoke("create_bot", { name: nameRef.current?.value, token: tokenRef.current?.value }) as Bot;
        dispatch(addBot(result));
        dispatch(setSettings({ current_bot: result.id }));
    }

    return (
        <>
            <HeaderText>Create a new bot</HeaderText>
            <label>Name:</label>
            <input ref={nameRef} />
            <label>Token:</label>
            <input ref={tokenRef} />
            <button onClick={createBot}>Create</button>
        </>
    );
}