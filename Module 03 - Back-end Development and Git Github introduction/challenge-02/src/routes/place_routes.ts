import { Router } from "express";
import { createPlace, deletePlace, getPlaceById, getPlaces, updatePlace } from "../controllers/places_controller"
const placeRouter = Router();


placeRouter.post("/", createPlace);
placeRouter.get("/all", getPlaces);
placeRouter.get("/unique/:place_id", getPlaceById);
placeRouter.delete("/:place_id", deletePlace);
placeRouter.patch("/:place_id", updatePlace);

export { placeRouter };
