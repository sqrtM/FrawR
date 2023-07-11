import React from "react";
import { Entity } from '../../../rustlib/pkg';
import StatusBarContainer from "./StatusBarContainer";

interface SideBarProps {
    playerInfo: Entity
}

export default function SideBar({playerInfo}: SideBarProps): React.JSX.Element {
    return (
        <div>
            <StatusBarContainer statusBars={playerInfo.status_bars}/>
        </div>
    )
}