import { Entity, Tile } from "rustlib";
import TileSpan from "./TileSpan";
import React, { memo } from "react";

type TileRowProps = {
  row: Tile[],
  tileIndex: number
  entities: Entity[]
}

const TileRow = memo(function TileRow(props: TileRowProps): React.JSX.Element {
  return (
    <>
      {
        props.row.map((i, index) => {
          /** @todo this is slow af */
          let entitiesForThisSpan = props.entities?.filter(i => i.location.x === index)
          return (
            <span
              key={"tile-" + i.location.x + "-" + i.location.y}
              id={"tile-" + i.location.x + "-" + i.location.y}
              style={
                entitiesForThisSpan !== undefined && entitiesForThisSpan.length > 0
                  ? { "color": `white` }
                  : { "color": `#${(i.val + 400)}` }
              }
            >
              <TileSpan tile={i} entities={entitiesForThisSpan} />
            </span>
          )
        })
      }
    </>
  )
}, areEqual)

function areEqual(prevProps: TileRowProps, nextProps: TileRowProps) {
  return prevProps.entities[0] == nextProps.entities[0]
}

export default TileRow;
