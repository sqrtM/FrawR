import { Entity, Tile } from "rustlib";
import TileSpan from "./TileSpan";
import React, { memo, } from "react";

type TileRowProps = {
  row: [[x: number, y: number], Tile][],
  tileIndex: number
  entity: Entity[]
  player: Entity | false
}

const TileRow = memo(function TileRow(props: TileRowProps): React.JSX.Element {

  //console.log("rerender check" + props.tileIndex)

  return (
    <>
      {
        props.row.map((i, index) => {
          let entitiesForThisSpan: Entity | false = props.entity?.find(i => i.location.x === index) || false
          let player = props.player && props.player.location.x === index ? props.player : false;
          return (
            <span
              key={"tile-" + i[0][0] + "-" + i[0][1]}
              id={"tile-" + i[0][0] + "-" + i[0][1]}
              style={
                entitiesForThisSpan !== undefined && entitiesForThisSpan || player
                  ? { "color": `white` }
                  : { "color": `#${(i[1].val + 400)}` }
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
  return a.entity.length === b.entity.length && a.entity.every((val, index) => val === b.entity[index]) && a.player === b.player;
}

export default TileRow;
