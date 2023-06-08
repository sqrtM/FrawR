import { Entity, Tile } from "rustlib";
import TileSpan from "./TileSpan";
import React, { memo, } from "react";

type TileRowProps = {
  row: Tile[],
  tileIndex: number
  entities: Entity[]
  player: Entity | false
}

const TileRow = memo(function TileRow(props: TileRowProps): React.JSX.Element {
  return (
    <>
      {
        props.row.map((i, index) => {
          let entitiesForThisSpan = props.entities?.filter(i => i.location.x === index)
          let player = props.player && props.player.location.x === index ? props.player : false;
          return (
            <span
              key={"tile-" + i.location.x + "-" + i.location.y}
              id={"tile-" + i.location.x + "-" + i.location.y}
              style={
                entitiesForThisSpan !== undefined && entitiesForThisSpan.length > 0 || player
                  ? { "color": `white` }
                  : { "color": `#${(i.val + 400)}` }
              }
            >
              <TileSpan tile={i} entities={entitiesForThisSpan} player={player} />
            </span>
          )
        })
      }
    </>
  )
}, areEqual)

function areEqual(a: TileRowProps, b: TileRowProps) {
  return a.entities.length === b.entities.length && a.entities.every((val, index) => val === b.entities[index]) && a.player === b.player;
}

export default TileRow;
