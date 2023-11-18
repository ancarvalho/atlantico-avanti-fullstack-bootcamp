import prisma from "../database/prisma_database";
import { Request, Response } from 'express';


const createCategory = async (req: Request, res: Response) => {

  try {
    const { name } = req.body;
    const category = await prisma.category.create({
      data: {
        name
      }
    }
    )
    res.status(201).json({ "status": "Category Created Successfully", "data": category });

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}

const getCategories = async (req: Request, res: Response) => {
  try {
    const categories = await prisma.category.findMany()
    res.status(200).json({ "status": "Categories Listed Successfully", "data": categories })

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}


const getCategoryById = async (req: Request, res: Response) => {
  try {
    const { category_id } = req.params;
    const category = await prisma.category.findFirst({
      where: {
        id: {
          equals: category_id
        }
      }
    })
    res.status(200).json({ "status": "Category Listed Successfully", "data": category })

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}


const deleteCategory = async (req: Request, res: Response) => {
  try {
    const { category_id } = req.params;
    await prisma.category.delete({
      where: {
        id: category_id
      }
    })

    res.status(200).json({ "status": "Category Deleted Successfully" })
  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}

const updateCategory = async (req: Request, res: Response) => {


  try {
    const { category_id } = req.params;
    const { name } = req.body;
    await prisma.category.update({
      where: {
        id: category_id
      },
      data: {
        name
      }
    })
    res.status(200).json({ "status": "Category Updated Successfully" })
  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}


export { getCategories, getCategoryById, createCategory, updateCategory, deleteCategory }


