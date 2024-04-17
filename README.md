# Relection Publisher-1

<li>
    <ol>Menurut saya, trait dibutuhkan karena observernya bisa lebih dari satu. Mungkin saya tiap observer memiliki behaviour yang berbeda-beda sehingga setiap behaviour butuh trait yang berbeda.</ol>
    <ol>DashMap lebih bagus. Setiap subscriber dapat dikelompokkan pada observernya masing-masing. Daftar subscriber juga dapat disimpan di Dashmap juga dengan key subscriber id agar mudah ditemukan.</ol>
    <ol>DashMap sendiri secara default thread safe jadi sudah cukup.</ol>
</li>

# Relection Publisher-2

<li>
    <ol>Setiap component harus punya single responsibility agar meningkatkan maintainability. Selain itu, single responsibility principle membuat program menjadi modular dan resuable.</ol>
    <ol>Menurut pandangan saya, nanti nya setiap model akan berinteraksi satu-sama lain secara langsung, tanpa melibatkan sebuah "universal repository". Tiap model harus mengimplementasikan repositorynya masing-masing berupa map, list, atau yang lainnya di dalam kelasnya. Hal ini tidak efektif karena model yang masih 1 jenis/memiliki banyak kesamaan harus ditaruh pada respository yang sama.</ol>
    <ol>Saya sangat menyukai postman. Saya sudah jatuh hati pada postman segera setelah saya bertemu untuk pertama kalinya dengannya. Postman banyak membantu dalam melakukan method GET, POST, PUT, dan DELETE untuk keperluan individu project maupun group project. Contohnya, kita dapat menentukan keseluruhan request body, parameter, dan juga menampilkan Response dengan mudah dibaca.</ol>
</li>