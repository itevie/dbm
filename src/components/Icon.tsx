import React from "react";
import { HTMLAttributes } from "react";

export default function Icon(props: { icon: string } & HTMLAttributes<HTMLImageElement>) {
    return (
        <img {...props} className={`icon ${props.className || ""}`} src={`/icons/${props.icon}.svg`} />
    );
}