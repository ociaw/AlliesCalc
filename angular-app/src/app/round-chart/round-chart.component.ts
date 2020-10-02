import { Component, Input, OnInit, ViewChild } from '@angular/core';
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
  styleUrls: ['./round-chart.component.scss']
})
export class RoundChartComponent implements OnInit {
  private _summaries: RoundSummary[] = [];
  chart: ApexChart = { type: "bar", height: 350, animations: { enabled: false } };
  title: ApexTitleSubtitle = { text: "IPC Chart" };
  plotOptions: ApexPlotOptions = { bar: { horizontal: false } };
  dataLabels: ApexDataLabels = { enabled: false };
  tooltip: ApexTooltip = { y: { formatter: val => val.toFixed(2) + " IPC" } };
  yaxis: ApexYAxis = { title: { text: "IPC" }, decimalsInFloat: 1 };
  xaxis: ApexXAxis = { categories: [],  };
  series: ApexAxisChartSeries;

  @ViewChild("chart", { static: false }) chartComponent!: ChartComponent;

  constructor() {
    this.series = [
      {
        name: "Attacker IPC",
        data: []
      },
      {
        name: "Defender IPC",
        data: []
      },
    ];
  }

  ngOnInit(): void {
  }


  @Input() set summaries(value: RoundSummary[]) {
    this._summaries = value;
    let categories = value.map(summary => summary.index == 0 ? "Pre-battle" : "Round " + summary.index);
    let attackerIpcSeries = value.map(summary => summary.attacker.ipc.mean);
    let defenderIpcSeries = value.map(summary => summary.defender.ipc.mean);

    this.xaxis = { categories: categories };
    this.series = [
      {
        name: "Attacker IPC",
        data: attackerIpcSeries,
        color: "#F00"
      },
      {
        name: "Defender IPC",
        data: defenderIpcSeries,
        color: "#088",
      }
    ];
  }
}
