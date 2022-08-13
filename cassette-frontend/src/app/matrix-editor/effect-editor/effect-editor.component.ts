import { Component, OnInit, Inject, EventEmitter, ViewChild } from '@angular/core';
import { ActivatedRoute } from '@angular/router'


import { Mixer } from 'src/app/mixer';
import { Effect, EffectsWrapper } from 'src/app/effect';
import { EffectOption } from 'src/app/effect-option';
import { AnalogOption } from 'src/app/effect-option';
import { BooleanOption } from 'src/app/effect-option';
import { ColorOption } from 'src/app/effect-option';
import { SelectOption } from 'src/app/effect-option';
import { EffectOptionType } from 'src/app/effect-option';
import {MatDialog, MatDialogRef, MAT_DIALOG_DATA} from '@angular/material/dialog';
import { ColorEvent } from 'ngx-color';
import { BackendService } from 'src/app/services/backend.service';
import { MatSelectChange } from '@angular/material/select';
import { on } from 'ws';
import { LiveViewComponent } from 'src/app/shared/live-view/live-view.component';

@Component({
  selector: 'effect-editor',
  templateUrl: './effect-editor.component.html',
  styleUrls: ['./effect-editor.component.css']
})
export class EffectEditorComponent implements OnInit {
    @ViewChild('live_view')
    live_view!: LiveViewComponent;

    effectsWrapper: EffectsWrapper = {activeId: 0, effects: []};
    untouchedEffectsWrapper: EffectsWrapper = {activeId: 0, effects: []};
    readonly EffectOptionType = EffectOptionType;
    currentEffect: Effect = {id: 0, name: "", options: []};
    output_id: number = 0;

    id: number = 0;
    mixer_id: number = 0;
    currentMixer: Mixer = {id: 0, options: []};
    untouchedMixer: Mixer = {id: 0, options: []};
    type: string = "";
    constructor(private route: ActivatedRoute, public dialog: MatDialog, private backend: BackendService) { }

    ngOnInit(): void {
        this.type = String(this.route.snapshot.paramMap.get('type'));
        this.id = Number(this.route.snapshot.paramMap.get('id'));
        this.output_id=Number(this.route.snapshot.paramMap.get('output_id'));
        if(this.type == "mixer") {
            this.backend.getMixer(this.id).subscribe(mixer => {
                this.currentMixer = mixer;
                this.untouchedMixer = JSON.parse(JSON.stringify(mixer));
            });
        }
        else if(this.type == "channel") {
            this.backend.getEffects(this.id).subscribe(effectsWrapper => {
                this.effectsWrapper = effectsWrapper
                this.untouchedEffectsWrapper = JSON.parse(JSON.stringify(this.effectsWrapper));
                this.currentEffect = this.effectsWrapper.effects.find(e => e.id == effectsWrapper.activeId)!;
            });
        }
    }

    public onActiveEffectChange(event: MatSelectChange) {
        this.effectsWrapper.activeId = event.value;
        this.backend.setChannelActiveEffect(this.id, this.effectsWrapper.activeId).subscribe(() => {

        });
    }

    public onEffectOptionChange() {
        if(this.type == "mixer") {
            this.backend.updateMixer(this.id, this.currentMixer).subscribe(() => {
            });
        }
        else if(this.type == "channel") {
            this.backend.updateChannelEffectOptions(this.id, this.currentEffect).subscribe(() => {
            });
        }
    }


    public onRevert() {
        console.log("Revert");
        if(this.type == "mixer") {
            this.currentMixer = JSON.parse(JSON.stringify(this.untouchedMixer));
            this.onEffectOptionChange();
        }
        else if(this.type == "channel") {
            this.effectsWrapper = JSON.parse(JSON.stringify(this.untouchedEffectsWrapper));
            this.currentEffect = this.effectsWrapper.effects.find(e => e.id == this.effectsWrapper.activeId)!;
            this.backend.setChannelActiveEffect(this.id, this.effectsWrapper.activeId).subscribe(() => {
                this.onEffectOptionChange();
            });
        }
    }

    public onApply() {
        console.log("Apply");
    }

    onImageSelected(image: string) {
        // call the live view to update the image
        // this.live_view.setActiveImage(image);
        
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
            this.onEffectOptionChange();
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
