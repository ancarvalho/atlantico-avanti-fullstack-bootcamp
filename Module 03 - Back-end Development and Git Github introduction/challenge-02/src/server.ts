import express from "express";
import { categoryRouter } from "./routes/category_routes";
import { eventRouter } from "./routes/event_routes";
import { placeRouter } from "./routes/place_routes";

const app = express();
const PORT = 42069;
app.use(express.json());

app.get('/',  (req, res) => {
  res.status(200).json({ "status": "Services UP" })
})
app.use("/category", categoryRouter);
app.use("/event", eventRouter);
app.use("/place", placeRouter);


app.listen(PORT, () => { console.log(`SERVER Running ON ${PORT}`) });