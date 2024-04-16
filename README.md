# Relection Publisher-1

<li>
    <ol>Menurut saya, trait dibutuhkan karena observernya bisa lebih dari satu. Mungkin saya tiap observer memiliki behaviour yang berbeda-beda sehingga setiap behaviour butuh trait yang berbeda.</ol>
    <ol>DashMap lebih bagus. Setiap subscriber dapat dikelompokkan pada observernya masing-masing. Daftar subscriber juga dapat disimpan di Dashmap juga dengan key subscriber id agar mudah ditemukan.</ol>
    <ol>DashMap sendiri secara default thread safe jadi sudah cukup.</ol>
</li>