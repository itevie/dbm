import { useEffect, useState } from 'react';
import Sidebar from './components/Sidebar';
import "./css/main.css";
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

function App() {
    const [currentPage, setCurrentPage] = useState<string>("home");
    const dispatch = useDispatch();

    useEffect(() => {
        (async () => {
            let settings = await invoke("get_options") as Settings;
            dispatch(setSettings(settings));
            let bots = await invoke<Bot[]>("get_all_bots");
            for (const bot of bots)
                dispatch(addBot(bot));

            listen("running_bots_update", data => {
                console.log(data);
            });

            listen("error", data => {
                console.error(data);
            })
        })();
    });

    return (
        <div className="app">
            <Sidebar setPage={name => setCurrentPage(name)} />
            <div className='app-content'>
                <Topbar page={currentPage} />
                <PageContainer>
                    {
                        {
                            "home": <CreateBot />,
                            "bots": <BotList />,
                            "commands": <CommandsPage />
                        }[currentPage]
                    }
                </PageContainer>
            </div>
        </div>
    );
}

export default App;
