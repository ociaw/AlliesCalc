import { Component, Input, ViewChild } from '@angular/core';
import { RoundSummary } from 'allies-calc-rs';
import {
  ApexAxisChartSeries,
  ApexChart,
  ChartComponent,
  ApexDataLabels,
  ApexPlotOptions,
  ApexYAxis,
  ApexXAxis,
  ApexTooltip, ApexTitleSubtitle
} from "ng-apexcharts";

@Component({
  selector: 'app-round-chart',
  templateUrl: './round-chart.component.html',
})
export class RoundChartComponent {
  private _summaries: RoundSummary[] = [];
  chart: ApexChart = { type: "bar", height: 350, animations: { enabled: false } };
  title: ApexTitleSubtitle = { text: "Stats by round" };
  plotOptions: ApexPlotOptions = { bar: { horizontal: false } };
  dataLabels: ApexDataLabels = { enabled: false };
  tooltip: ApexTooltip = { y: { formatter: val => val.toFixed(2) } };
  yaxis: ApexYAxis = { decimalsInFloat: 1 };
  xaxis: ApexXAxis = { categories: [],  };
  series: ApexAxisChartSeries = []

  @ViewChild("chart", { static: false }) chartComponent!: ChartComponent;

  @Input() set summaries(value: RoundSummary[]) {
    this._summaries = value;
    let categories = value.map(summary => summary.index == 0 ? "Pre-battle" : "Round " + summary.index);

    this.xaxis = { categories: categories };
    this.series = [
      {
        name: "Attacker IPC",
        data: value.map(summary => summary.attacker.ipc.mean),
        color: "#F44"
      },
      {
        name: "Defender IPC",
        data: value.map(summary => summary.defender.ipc.mean),
        color: "#0AA",
      },
      {
        name: "Attacker Units",
        data: value.map(summary => summary.attacker.unit_count.mean),
        color: "#F00"
      },
      {
        name: "Defender Units",
        data: value.map(summary => summary.defender.unit_count.mean),
        color: "#088",
      },
      {
        name: "Attacker Strength",
        data: value.map(summary => summary.attacker.strength.mean),
        color: "#C00"
      },
      {
        name: "Defender Strength",
        data: value.map(summary => summary.defender.strength.mean),
        color: "#066",

      }
    ];
  }
}
