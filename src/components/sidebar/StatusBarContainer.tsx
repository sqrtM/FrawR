import React from "react";
import { StatusBar as StatusBarInfo, StatusBars } from "rustlib"
import StatusBar from "./StatusBar";

interface StatusBarContainerProps {
    statusBars: StatusBars
}

export default function StatusBarContainer({ statusBars }: StatusBarContainerProps): React.JSX.Element {

    let ea: any = {};
    function constructStatusBars(i: any, e: any = ea, prefix: string = '') {
        for (let [key, value] of Object.entries(i)) {
            if (prefix) {
                key = prefix + key
            }
            if (key == "mana") {
                constructStatusBars(value, e, "mana:")
            } else {
                ea = { ...ea, [key]: value }
            }
        }

    }
    constructStatusBars(statusBars)

    return (
        <div id="status-bars" >
            {
                Object.entries(ea).map((i, index) => {
                    return <div key={"statusbar" + index}><StatusBar name={i[0] as string} barInfo={i[1] as StatusBarInfo} /></div>
                })
            }
        </div>
    )
}