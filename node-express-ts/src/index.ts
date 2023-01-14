import express from "express";
import * as routes from "./routes";
import {AddressInfo} from "net";

const app = express();
app.use(express.json());
routes.register(app);

const server = app.listen(5000, '0.0.0.0', () => {
    const {port, address} = server.address() as AddressInfo
    console.log('Server listening on: ', "http://"+ address + ":" + port);
});