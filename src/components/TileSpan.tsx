import React, { memo } from "react";
import { Entity, Tile } from "rustlib";

type TileSpanProps = {
  tile: Tile,
  entities: Entity[]
}

const TileSpan = memo(function TileSpan(props: TileSpanProps): React.JSX.Element {
  console.log("rerendered" + "_" + props.tile.location.x + "-" + props.tile.location.y)
  return (
    <>
      {
      props.entities.length > 0 
        ? props.entities[0].char 
        : props.tile.char
      }
    </>
  )
}, areEqual)

function areEqual(prevProps: TileSpanProps, nextProps: TileSpanProps) {
  return prevProps.entities[0] == nextProps.entities[0]
}

export default TileSpan;
