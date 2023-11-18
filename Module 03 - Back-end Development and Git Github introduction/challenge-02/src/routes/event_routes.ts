import { Router } from "express";
import { createEvent, deleteEvent, getEventById, getEvents, getEventsFiltered, updateEvent } from "../controllers/event_controller"
const eventRouter = Router();


eventRouter.post("/", createEvent);
eventRouter.get("/all", getEvents);
eventRouter.get("/unique/:event_id", getEventById);
eventRouter.get("/find", getEventsFiltered);
eventRouter.delete("/:event_id", deleteEvent);
eventRouter.patch("/:event_id", updateEvent);
export { eventRouter };
