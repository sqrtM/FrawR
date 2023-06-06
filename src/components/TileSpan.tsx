import React, { memo } from "react";
import { Entity, Tile } from "rustlib";

type TileSpanProps = {
  tile: Tile,
  entities: Entity[]
}

const TileSpan = memo(function TileSpan(props: TileSpanProps): React.JSX.Element {
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

function areEqual(a: TileSpanProps, b: TileSpanProps) {
  return a.entities.length === b.entities.length && a.entities.every((val, index) => val === b.entities[index]);
}

export default TileSpan;
