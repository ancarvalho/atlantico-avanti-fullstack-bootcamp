import { PrismaClient } from "@prisma/client";



const prisma = new PrismaClient({
  log: ["error", "info", "query", "warn"],
}).$extends({

  query: {
    async $allOperations({ operation, model, args, query }) {
      const start = performance.now()
      const result = await query(args)
      const end = performance.now()
      const time = end - start
      console.log(`query took ${time.toFixed(0)}ms`)
      return result
    },
  },
})

export default prisma;
