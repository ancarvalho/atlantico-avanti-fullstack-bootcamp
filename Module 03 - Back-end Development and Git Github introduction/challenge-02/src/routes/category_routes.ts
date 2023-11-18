import { Router } from "express";
import { createCategory, deleteCategory, getCategories, getCategoryById, updateCategory } from "../controllers/category_controller"

const categoryRouter = Router();


categoryRouter.post("/", createCategory);
categoryRouter.get("/all", getCategories);
categoryRouter.get("/unique/:category_id", getCategoryById);
categoryRouter.delete("/:category_id", deleteCategory);
categoryRouter.put("/:category_id", updateCategory);

export { categoryRouter };
