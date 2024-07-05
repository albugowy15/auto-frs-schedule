# Changelog

## [2.8.1](https://github.com/albugowy15/auto-frs-schedule/compare/v2.8.0...v2.8.1) (2024-07-05)


### Bug Fixes

* avoid print database url when opening database connection ([d782a01](https://github.com/albugowy15/auto-frs-schedule/commit/d782a013187608c413a41144761caf0ae14d59ad))

## [2.8.0](https://github.com/albugowy15/auto-frs-schedule/compare/v2.7.0...v2.8.0) (2024-06-13)


### Features

* add parse strategy for class that has "(EN) - T - IUP" substring ([480215a](https://github.com/albugowy15/auto-frs-schedule/commit/480215ac52dc9f7b6592a77abbe50d73b9b0ada2))
* better parse strategy for different cases ([8e55adc](https://github.com/albugowy15/auto-frs-schedule/commit/8e55adc4ead6e7cea183e7c9c1e0d1df00377d55))
* class repository ([78edc6d](https://github.com/albugowy15/auto-frs-schedule/commit/78edc6d71069e23cc0bb74ab637c937bcb5ef6c5))
* clean command ([fcd6f61](https://github.com/albugowy15/auto-frs-schedule/commit/fcd6f6165368f2da54622499ed380fe8c06a768c))
* cli parser ([08d2af2](https://github.com/albugowy15/auto-frs-schedule/commit/08d2af22fa56403df028ce40eef5c342c89c4b39))
* compare with latest schedule ([813922a](https://github.com/albugowy15/auto-frs-schedule/commit/813922aa4d0b84bcf449d481c36c3a7d731cdded))
* db connection ([3b8e79a](https://github.com/albugowy15/auto-frs-schedule/commit/3b8e79a289206e6a4ea2736bebcd096f59120d03))
* delete rows and insert data to Class ([7cddcd6](https://github.com/albugowy15/auto-frs-schedule/commit/7cddcd6114dae3ca14e55a8deb8b660e9d2d042f))
* find class schedule command ([5127976](https://github.com/albugowy15/auto-frs-schedule/commit/512797658d8576d4c2ea59846ee3ffc924a7927c))
* helpfully log process ([0630934](https://github.com/albugowy15/auto-frs-schedule/commit/06309347a47190448af09500336b1ecc63a6f4ae))
* insert non-classes subject ([d72137b](https://github.com/albugowy15/auto-frs-schedule/commit/d72137bc0848a85c059b17371529f2e45305d9cb))
* LecturerSubjectSessionMap struct ([0ff90b3](https://github.com/albugowy15/auto-frs-schedule/commit/0ff90b39deb2ff120c8d43af8651567d2d7a81a4))
* more informatif log message and simplify task join ([6be2be2](https://github.com/albugowy15/auto-frs-schedule/commit/6be2be26e36355700689cc151e1d4e387b40fde1))
* organize module with lib.rs ([62be1f7](https://github.com/albugowy15/auto-frs-schedule/commit/62be1f754acb1d56d606e1c7ee496616aceb7eb2))
* parse subject and code ([57eabe0](https://github.com/albugowy15/auto-frs-schedule/commit/57eabe046e2475148f1aad00c47a3076762835be))
* parse team teaching class ([23d7348](https://github.com/albugowy15/auto-frs-schedule/commit/23d7348af229324f760b69b4ecb986c29a1e0179))
* print with stdout ([ecd2ba3](https://github.com/albugowy15/auto-frs-schedule/commit/ecd2ba332fcc4bf7cffeff7e3ddf81e12754321b))
* progress indicator ([12b4081](https://github.com/albugowy15/auto-frs-schedule/commit/12b4081103d25fda397598da0cdf8ae831e3641b))
* propagate log error inside command handler ([2030d24](https://github.com/albugowy15/auto-frs-schedule/commit/2030d2445562e9d7f6b1f1727af1f1b0793b0367))
* read from xlsx ([92040ab](https://github.com/albugowy15/auto-frs-schedule/commit/92040ab71cfd3b9d93b545f00c141ef45d869e39))
* remove excel.rs ([165ca73](https://github.com/albugowy15/auto-frs-schedule/commit/165ca73dccc5bd24068b561bdbb01f7467917fda))
* retrieve id from database ([54e6635](https://github.com/albugowy15/auto-frs-schedule/commit/54e6635757ae25140badeab7a19a1c7360871ccb))
* retrive class detail from excel ([e034c66](https://github.com/albugowy15/auto-frs-schedule/commit/e034c666a87fba4ae415f5988fbbf5a42fbcab27))
* simplify command ([6728473](https://github.com/albugowy15/auto-frs-schedule/commit/67284732770971b8dbea488c040c6215fe9bafbe))
* sync command ([774e3cf](https://github.com/albugowy15/auto-frs-schedule/commit/774e3cf4a23f352ef19e45c8f0829483a741bffc))
* update parse strategy based on new schedule format ([e1ce815](https://github.com/albugowy15/auto-frs-schedule/commit/e1ce8151602b010d46d568cdfe0cb89d4e1d5be3))
* use log crate for logging ([dac0f33](https://github.com/albugowy15/auto-frs-schedule/commit/dac0f33755a254da4ced304bd57884af2cbf44b1))


### Bug Fixes

* compare subject name in lowercase ([f4e6d2e](https://github.com/albugowy15/auto-frs-schedule/commit/f4e6d2e112a9b9d39b2e1ec3919cb59f6bc844cd))
* comparing lecturers code ([33f0619](https://github.com/albugowy15/auto-frs-schedule/commit/33f0619c8225cc413cc0db7ff271f53b94e1b469))
* Fix error handling in clean and update handlers ([929673b](https://github.com/albugowy15/auto-frs-schedule/commit/929673b6c199009715354c1154647ac179683a7e))
* fix type error after upgrade calamine to 0.24.0 ([4d1b78f](https://github.com/albugowy15/auto-frs-schedule/commit/4d1b78fed22f769a34f350b06c5a0c3a43508bcf))
* handle error by output the error into console ([7619b2d](https://github.com/albugowy15/auto-frs-schedule/commit/7619b2d6bda05e87b402389d9c4c83aa25d0adbd))
* insert class with multiple lecturers ([e5820c8](https://github.com/albugowy15/auto-frs-schedule/commit/e5820c845dc7e58b515c8beb94434b2f62067df4))
* parse team teaching and iup akselerasi ([4d95908](https://github.com/albugowy15/auto-frs-schedule/commit/4d95908b7be1f61d074f9ed3d593fbe098ce5d73))
* PTEIC subject not included when parsing excel ([49ca33c](https://github.com/albugowy15/auto-frs-schedule/commit/49ca33cef61ffdd7064b77ffd7813397048d0736))
* sum totalSks error decode decimal type into int ([3908be8](https://github.com/albugowy15/auto-frs-schedule/commit/3908be8b00b48bf58be4ce7a0a9af4d522ed423a))
* Update version and clean up code ([103e212](https://github.com/albugowy15/auto-frs-schedule/commit/103e21294723bdd072400dcb56310796bc6b8412))
* use try_get() to safely decode sql result ([d229371](https://github.com/albugowy15/auto-frs-schedule/commit/d22937193a1e459ba4f94904552568d01c0ccbc1))


### Performance Improvements

* concurrent task with tokio spawn and try_join ([c6c2dc5](https://github.com/albugowy15/auto-frs-schedule/commit/c6c2dc57ba2add30ecfd7d00822dd74b5744bd09))
* open and close db connection inside handler function ([af86c99](https://github.com/albugowy15/auto-frs-schedule/commit/af86c99d197a2d023760321d0761d4311ebe3728))
* remove hashmap new initialization ([5da5f3a](https://github.com/albugowy15/auto-frs-schedule/commit/5da5f3aeddb9e12e65fc8624f7961be9848fbe63))
* small optimization ([680c872](https://github.com/albugowy15/auto-frs-schedule/commit/680c872bce063b5f0fe100a7c35c1b5605995f03))

## [2.7.0](https://github.com/albugowy15/auto-frs-schedule/compare/v2.6.2...v2.7.0) (2024-06-13)


### Features

* add parse strategy for class that has "(EN) - T - IUP" substring ([480215a](https://github.com/albugowy15/auto-frs-schedule/commit/480215ac52dc9f7b6592a77abbe50d73b9b0ada2))
* find class schedule command ([5127976](https://github.com/albugowy15/auto-frs-schedule/commit/512797658d8576d4c2ea59846ee3ffc924a7927c))
* more informatif log message and simplify task join ([6be2be2](https://github.com/albugowy15/auto-frs-schedule/commit/6be2be26e36355700689cc151e1d4e387b40fde1))
* print with stdout ([ecd2ba3](https://github.com/albugowy15/auto-frs-schedule/commit/ecd2ba332fcc4bf7cffeff7e3ddf81e12754321b))
* remove excel.rs ([165ca73](https://github.com/albugowy15/auto-frs-schedule/commit/165ca73dccc5bd24068b561bdbb01f7467917fda))


### Bug Fixes

* handle error by output the error into console ([7619b2d](https://github.com/albugowy15/auto-frs-schedule/commit/7619b2d6bda05e87b402389d9c4c83aa25d0adbd))
* PTEIC subject not included when parsing excel ([49ca33c](https://github.com/albugowy15/auto-frs-schedule/commit/49ca33cef61ffdd7064b77ffd7813397048d0736))
* sum totalSks error decode decimal type into int ([3908be8](https://github.com/albugowy15/auto-frs-schedule/commit/3908be8b00b48bf58be4ce7a0a9af4d522ed423a))
* use try_get() to safely decode sql result ([d229371](https://github.com/albugowy15/auto-frs-schedule/commit/d22937193a1e459ba4f94904552568d01c0ccbc1))
