import {Injectable} from '@angular/core';
import {HttpClient, HttpHeaders} from '@angular/common/http';
import {Observable} from 'rxjs';
import {Msg} from './msg';

@Injectable({
  providedIn: 'root'
})
export class ValidateService {

  url = 'https://basejumper_backend.challenge.kdctf.at';

  constructor(private http: HttpClient) {
  }

  validate_solution(msg: Msg): Observable<Msg> {
    const httpOptions = {
      headers: new HttpHeaders({
        'Content-Type': 'application/json',
      })
    };
    return this.http.post<Msg>(this.url + '/', msg, httpOptions);
  }
}
