const parseQueryParams = (param: any): string[] | undefined => {
  if (typeof param === 'string') {
    return param.split(",");
  }
  return undefined;
}


const checkValidDate = (date: any): Date | undefined => {
  if (date instanceof Date && !isNaN(Number(date))) {
    return date
  }
  return undefined
}

const parseDates = (dates: string[] | undefined): Date[] => {

  if (typeof dates === "undefined") {
    return []
  }
  let parsedDates: Date[] = [];

  const datesSize = dates.length > 2 ? 2 : dates.length;
  for (let i = 0; i < datesSize; i++) {


    const validDate = checkValidDate(new Date(dates[i]));

    if (typeof validDate === "undefined") {
      return []
    }
    parsedDates.push(validDate);
  }
  return parsedDates
}

export { parseQueryParams, parseDates }