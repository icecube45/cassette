import { Component, OnInit, Inject } from '@angular/core';
import { ActivatedRoute } from '@angular/router'

import { EFFECTS } from '../../mock-effects';

import { Effect } from 'src/app/effect';
import { EffectOption } from 'src/app/effect-option';
import { AnalogOption } from 'src/app/effect-option';
import { BooleanOption } from 'src/app/effect-option';
import { ColorOption } from 'src/app/effect-option';
import { SelectOption } from 'src/app/effect-option';
import { EffectOptionType } from 'src/app/effect-option';
import {MatDialog, MatDialogRef, MAT_DIALOG_DATA} from '@angular/material/dialog';
import { ColorEvent } from 'ngx-color';

@Component({
  selector: 'effect-editor',
  templateUrl: './effect-editor.component.html',
  styleUrls: ['./effect-editor.component.css']
})
export class EffectEditorComponent implements OnInit {

    effects = EFFECTS;
    readonly EffectOptionType = EffectOptionType;
    currentEffect: Effect = this.effects.find(e => e.active == true)!;
    channel_id: number = 0;
    mixer_id: number = 0;
    constructor(private route: ActivatedRoute, public dialog: MatDialog) { }

    ngOnInit(): void {
        this.channel_id = Number(this.route.snapshot.paramMap.get('channel_id'));
        this.mixer_id = Number(this.route.snapshot.paramMap.get('mixer_id'));
    }

    onRevert() {
        console.log("Revert");
    }

    onApply() {
        console.log("Apply");
        console.log(JSON.stringify(this.effects[0]));
    }



    hexToRgb(hex: string) {
        const shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
        hex = hex.replace(shorthandRegex, (m, r, g, b) => {
          return r + r + g + g + b + b;
        });
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
          r: parseInt(result[1], 16),
          g: parseInt(result[2], 16),
          b: parseInt(result[3], 16)
        } : null;
      }    

    openColorPicker(option: ColorOption) {
        const dialogRef = this.dialog.open(ColorPickerDialog, {
            width: '90vw',
            data: option,
          });
      
          dialogRef.afterClosed().subscribe(result => {
            console.log('The dialog was closed');
          });
        // option.colorPickerOpen = true;
    }

    formatLabel(value: number) {
        if (value >= 1000) {
          return Math.round(value / 1000) + 'k';
        }
    
        return value;
      }
}

@Component({
    selector: 'color-picker-dialog',
    templateUrl: 'color-picker-dialog.html',
    styleUrls: ['color-picker-dialog.css']

  })
  export class ColorPickerDialog {
    constructor(
      public dialogRef: MatDialogRef<ColorPickerDialog>,
      @Inject(MAT_DIALOG_DATA) public data: ColorOption,
    ) {}
  
    handleChangeComplete($event: ColorEvent) {
        this.data.value = $event.color.hex;
    }
  }
