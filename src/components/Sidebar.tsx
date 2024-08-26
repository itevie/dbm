import React from "react";
import "../css/sidebar.css";
import Icon from "./Icon";

export default function Sidebar(props: { setPage: (name: string) => void }) {
    return (
        <div className="sidebar">
            {
                [
                    ["home", "home"],
                    ["robot", "bots"],
                    [],
                    ["robot", "bot-settings"],
                    ["code", "commands"]
                ].map(part => part[0]
                    ? <Icon icon={part[0]} className="sidebar-icon" onClick={() => props.setPage(part[1])} />
                    : <hr />)
            }
            <div className="sidebar-bottom">
                <Icon icon="reload" className="sidebar-icon" onClick={() => window.location.reload()} />
            </div>
        </div>
    );
}