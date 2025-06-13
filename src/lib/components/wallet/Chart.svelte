<script>
    import { afterUpdate, onDestroy } from 'svelte';
    import { createChart } from 'lightweight-charts';
    import { chartEmptyData } from '$lib/scripts/mock-data';

    let chart;
    let lineSeries;
    let resizeObserver;

    const initChart = () => {
        if (chart) {
            chart.remove();
        }
        const chartElement = document.getElementById('chart');
        if (chartElement) {
            chart = createChart(chartElement);
            chart.applyOptions({
                layout: {
                    background: '#181818',
                    textColor: '#fff',
                },
                grid: {
                    vertLines: {
                        color: '#80808080',
                    },
                    horzLines: {
                        color: '#80808080',
                    },
                },
            });
            lineSeries = chart.addAreaSeries();
            //const data = chartData();
            const data = chartEmptyData();
            lineSeries.setData(data);
            lineSeries.applyOptions({
                lineColor: '#FD4102',
                lineWidth: 2,
            });
        }
    };

    afterUpdate(() => {
        initChart();
        
        // Create resize observer
        resizeObserver = new ResizeObserver(() => {
            initChart();
        });

        // Start observing the chart element
        const chartElement = document.getElementById('chart');
        if (chartElement) {
            resizeObserver.observe(chartElement);
        }
    });

    onDestroy(() => {
        // Clean up resize observer
        if (resizeObserver) {
            resizeObserver.disconnect();
        }
        if (chart) {
            chart.remove();
        }
    });
</script>

<div class="w-auto h-80 bg-dark mx-4 mb-8" id="chart">
</div>