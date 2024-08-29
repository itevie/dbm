import { useEffect, useState } from 'react';
import Sidebar from './components/Sidebar';
import "./css/main.css";
import "./css/models.css";
import PageContainer from './components/PageContainer';
import CreateBot from './pages/CreateBot';
import React from 'react';
import Topbar from './components/Topbar';
import BotList from './pages/BotList';
import CommandsPage from './pages/Commands';
import { Bot, Settings } from './types/structures';
import { invoke } from '@tauri-apps/api';
import { useDispatch } from 'react-redux';
import { setSettings } from './stores/options';
import { addBot } from './stores/bots';
import { listen } from '@tauri-apps/api/event';
import { setRunningBots } from './stores/runningBots';
import { TauriEvents } from './types/tauri';
import ModelManager, { pushModel } from './components/models/ModelManager';
import { errorMessage, messageModel } from './components/models/modelTemplates';
import BotSettings from './pages/BotSettings';
import Button from './components/Button';

export let save: (() => void) | null = null;
export let setSave: (cb: () => void) => void = () => { };

function App() {
    const [currentPage, setCurrentPage] = useState<string>("home");
    const dispatch = useDispatch();
    const [showSave, setShowSave] = useState<boolean>(false);

    useEffect(() => {
        (async () => {
            setSave = cb => {
                save = cb;
                setShowSave(true);
            }

            let settings = await invoke("get_options") as Settings;
            dispatch(setSettings(settings));
            let bots = await invoke<Bot[]>("get_all_bots");
            for (const bot of bots)
                dispatch(addBot(bot));

            listen<TauriEvents["running_bots_update"]>("running_bots_update", data => {
                dispatch(setRunningBots(data.payload.list));
            });

            listen<TauriEvents["error"]>("error", data => {
                errorMessage(data.payload.error.message);
            })
        })();
    }, []);

    function setCurrentPage2(name: string) {
        if (showSave) {
            return pushModel({
                title: "You have unsaved changes",
                body: "What would you like to do with these changes?",
                buttons: [
                    {
                        name: "Discard",
                        type: "error",
                        onClick: d => {
                            save = null;
                            setShowSave(false);
                            setCurrentPage(name);
                            d.close();
                        }
                    },
                    {
                        name: "Save",
                        type: "success",
                        onClick: async d => {
                            if (save) await save();
                            save = null;
                            setShowSave(false);
                            setCurrentPage(name);
                            d.close();
                        }
                    }
                ]
            })
        }
        setCurrentPage(name);
    }

    async function saveChanges() {
        if (save) await save();
        save = null;
        setShowSave(false);
    }

    return (
        <div className="app">
            <Sidebar setPage={name => setCurrentPage2(name)} />
            <div className='app-content'>
                <Topbar page={currentPage} />
                <PageContainer>
                    {
                        {
                            "home": <CreateBot />,
                            "bots": <BotList />,
                            "commands": <CommandsPage />,
                            "bot-settings": <BotSettings />
                        }[currentPage]
                    }
                </PageContainer>
            </div>
            {
                showSave && <Button onClick={saveChanges} style={{ position: "absolute", bottom: "15px", right: "15px", boxShadow: "0px 0px 20px 0px black" }}>Save</Button>
            }
        </div>
    );
}

export default App;
