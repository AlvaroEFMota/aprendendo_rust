pub fn merge_sort(v: &mut [i32], start: usize, end: usize) {
    if end-start > 1 {
        let middle = (start+end)/2;
        merge_sort(v, start, middle);
        merge_sort(v, middle, end);

        let mut aux_j = vec![0; middle-start]; // To create an empty vector, just use vec![]
        let mut aux_k = vec![0; end-middle];

        for i in 0..(middle-start) { // Encontrar uma melhor forma de copiar os dados
            aux_j[i] = v[i+start];
        }

        for i in 0..(end-middle) {
            aux_k[i] = v[middle+i];
        }
        
        let mut j = 0;
        let mut k = 0;
        let mut index_i: usize = 0;
        for i in start..end { // Aprender a colocar condições no for
            if j >= middle-start || k >= end-middle {
                break;
            }

            if aux_j[j] < aux_k[k] {
                v[i] = aux_j[j];
                j += 1;
            } else {
                v[i] = aux_k[k];
                k += 1;
            }
            index_i = i;
        }
        index_i += 1;

        for p in j..(middle-start) {
            v[index_i] = aux_j[p];
            index_i += 1;
        }

        for p in k..(end-middle) {
            v[index_i] = aux_k[p];
            index_i += 1;
        }
    }
}