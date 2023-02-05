// This file contains a demo of the new alpha API's these are stable to use but there may be rough edges.
/** @jsxImportSource solid-js */

import { fetchLink, initRspc } from "@rspc/client";
import { fetchLink as fetchLink2 } from "@rspc/client/full";
import { Procedures } from "../../../bindings";

// TODO: How are user defined links going to work with the whole full vs lite client??? -> Have type hint so they can assert they work with one or both

const rspc = initRspc<Procedures>().use(
  // fetchLink({
  //   url: "http://localhost:4000/rspc",
  // })
  // `fetchLink2` is built with full `Observable`'s. This is just a temporary mock for the websocket transport right now.
  fetchLink2({
    url: "http://localhost:4000/rspc",
  })
);

export default function AlphaPage() {
  // TODO: Don't copy this and use the vanilla client in SolidJS. It's not gonna end well in a more complicated app.
  rspc.query(["basic.echo", "Hello World"]).then(console.log);

  return <h1>Alpha</h1>;
}
