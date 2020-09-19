import { Component, Input, OnInit, Output } from '@angular/core';
import { FormArray, FormBuilder, FormControl, FormGroup } from '@angular/forms';
import { Unit } from '../model/unit';

let nextId = 0;

@Component({
  selector: 'app-setup-force',
  templateUrl: './setup-force.component.html',
  styleUrls: ['./setup-force.component.scss']
})
export class SetupForceComponent {
  private _units: Unit[] = [];
  @Input() title = "";
  setupForm: FormGroup;
  unitControls = new FormArray([]);
  id: String;

  constructor(formBuilder: FormBuilder) {
    this.id = `setup-force-${nextId++}-`;
    this.setupForm = formBuilder.group({
      lossOrderControl: new FormControl(''),
      unitControls: new FormArray([]),
    });
  }

  @Input() set units(value: Unit[]) {
    this._units = value;
    this.unitFormArray.clear();
    this._units.forEach(_ => {
      this.unitFormArray.push(new FormControl(0));
    });
  }

  get units() {
    return this._units;
  }

  @Output() get selectedUnits(): [Unit, number][] {
    let array: [Unit, number][] = [];
    this._units.forEach((unit, index) => {
      let count = parseInt(this.unitFormArray.at(index).value);
      if (count >= 1) {
        array.push([unit, count]);
      }
    });
    return array;
  }

  get unitFormArray() {
    return this.setupForm.controls.unitControls as FormArray;
  }
}
