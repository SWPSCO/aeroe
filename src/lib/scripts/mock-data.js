export const chartEmptyData = () => {
    const data = [];
    let genesisDate = new Date('2025-05-21T19:19:00-06:00');
    
    let iterDate = new Date(genesisDate.getTime()); // Start iterating from a copy of genesisDate
    const today = new Date(); // Current date/time will be the upper limit (inclusive)
    const value = 0.0; // Value for these data points

    // Loop from genesisDate up to today (inclusive), adding a data point for each day
    while (iterDate <= today) {
        data.push({
            time: iterDate.toISOString().split('T')[0],
            value: parseFloat(value.toFixed(2))
        });
        iterDate.setDate(iterDate.getDate() + 1); // Increment to the next day
    }
    return data;
}