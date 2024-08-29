import React, { HTMLAttributes } from "react";
import { ReactNode } from "react";

export default function HeaderText({ children, ...props }: { children: ReactNode } & HTMLAttributes<HTMLLabelElement>) {
    return (
        <label {...props} role="heading" className={`header ${props.className || ""}`}>
            {children}
        </label>
    );
}