import prisma from "../database/prisma_database";
import { Request, Response } from 'express';


const createPlace = async (req: Request, res: Response) => {

  try {
    const { name, address, neighborhood, city, state, country } = req.body;
    const place = await prisma.place.create({
      data: {
        name, address, neighborhood, city, state, country
      }
    }
    )
    res.status(201).json({ "status": "Place Created Successfully", "data": place });

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}

const getPlaces = async (_req: Request, res: Response) => {
  try {
    const places = await prisma.place.findMany()
    res.status(200).json({ "status": "Places Listed Successfully", "data": places })

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}


const getPlaceById = async (req: Request, res: Response) => {
  try {
    const { place_id } = req.params;
    const place = await prisma.place.findFirst({
      where: {
        id: {
          equals: place_id
        }
      }
    })
    res.status(200).json({ "status": "Place Listed Successfully", "data": place })

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}


const deletePlace = async (req: Request, res: Response) => {
  try {
    const { place_id } = req.params;
    await prisma.place.delete({
      where: {
        id: place_id
      }
    })

    res.status(200).json({ "status": "Place Deleted Successfully" })
  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}

const updatePlace = async (req: Request, res: Response) => {


  try {
    const { place_id } = req.params;
    const { name, address, city, state, country } = req.body;
    const place = await prisma.place.update({
      where: {
        id: place_id
      },
      data: {
        name, address, city, state, country
      }
    });
    res.status(200).json({ "status": "Place Updated Successfully", "data": place });

  } catch (error) {
    res.status(500).json({ "status": "An Error Ocurred On Server", error });
  }
}


export { getPlaces, getPlaceById, createPlace, updatePlace, deletePlace }


