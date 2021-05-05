import {Component, OnInit} from '@angular/core';
import {ValidateService} from './validate.service';
import {Msg} from './msg';


@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.css']
})
export class AppComponent implements OnInit {
  title = 'basejumper';
  passcode;

  motivational = [
    'Go go go, you can do it!', 'Soon we will reach the top.', ' Let\'s not give up yet!', 'Nice view from up here, right?',
    'You might be the first one to solve this challenge!', 'Come on, a little further!'
  ];
  currentText = 'Ready for the climb up? Lets start out easy. Try putting this string into the Input field below.';
  currentChallenge = '922FE339AE85E0950EBA469674AE71411F10FCC2C0626A9E52C46772A04D31A9';
  picture = '1';


  constructor(private validateService: ValidateService) {
  }

  ngOnInit(): void {
  }

  async checkPassPhrase(value: string): Promise<void> {
    const msg = new Msg(value);
    let res: Msg;
    try {
      res = await this.validateService.validate_solution(msg).toPromise();
    } catch (e) {
      res = new Msg('Please stop ddosing me. T_T', true);
    }

    if (res.err) {
      this.picture = '2';
      this.currentText = res.msg;
    } else {
      this.picture = '1';
      this.currentText = this.motivational[Math.floor(Math.random() * this.motivational.length)];
      this.currentChallenge = res.msg;
    }
  }
}

