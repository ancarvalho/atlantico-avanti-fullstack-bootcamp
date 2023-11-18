import { PrismaClient } from '@prisma/client'
import seed from "./seed.json";

const prisma = new PrismaClient()

const createCategories = async () => {
  await Promise.all(seed.categories.map(n => prisma.category.create({ data: { ...n } })))
    .then(() => console.log("Categories Created"))
    .catch((e) => console.log("Error on Creating Categories", e))
}

const createPlaces = async () => {
  await Promise.all(seed.places.map(n => prisma.place.create({ data: { ...n } })))
    .then(() => console.log("Places Created"))
    .catch((e) => console.log("Error on Creating Places", e))
}

const createEvents = async () => {
  await Promise.all(seed.events.map(n => prisma.event.create({ data: { ...n, date: new Date(n.date) } })))
    .then(() => console.log("Events Created"))
    .catch((e) => console.log("Error on Creating Events", e))
}


const createFromSeed = async () => {
  await createCategories();
  await createPlaces();
  await createEvents();
}

createFromSeed()