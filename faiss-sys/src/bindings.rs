/* automatically generated by rust-bindgen */

pub type idx_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissRangeSearchResult_H {
    _unused: [u8; 0],
}
pub type FaissRangeSearchResult = FaissRangeSearchResult_H;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissIDSelector_H {
    _unused: [u8; 0],
}
pub type FaissIDSelector = FaissIDSelector_H;
pub const FaissMetricType_METRIC_INNER_PRODUCT: FaissMetricType = 0;
pub const FaissMetricType_METRIC_L2: FaissMetricType = 1;
/// Some algorithms support both an inner product version and a L2 search version.
pub type FaissMetricType = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissIndex_H {
    _unused: [u8; 0],
}
pub type FaissIndex = FaissIndex_H;
extern "C" {
    pub fn faiss_Index_free(obj: *mut FaissIndex);
}
extern "C" {
    pub fn faiss_Index_d(arg1: *const FaissIndex) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Index_is_trained(arg1: *const FaissIndex) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Index_ntotal(arg1: *const FaissIndex) -> idx_t;
}
extern "C" {
    pub fn faiss_Index_metric_type(arg1: *const FaissIndex) -> FaissMetricType;
}
extern "C" {
    /// Perform training on a representative set of vectors
    ///
    /// @param index  opaque pointer to index object
    /// @param n      nb of training vectors
    /// @param x      training vecors, size n * d
    pub fn faiss_Index_train(
        index: *mut FaissIndex,
        n: idx_t,
        x: *const f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Add n vectors of dimension d to the index.
    ///
    /// Vectors are implicitly assigned labels ntotal .. ntotal + n - 1
    /// This function slices the input vectors in chuncks smaller than
    /// blocksize_add and calls add_core.
    /// @param index  opaque pointer to index object
    /// @param x      input matrix, size n * d
    pub fn faiss_Index_add(
        index: *mut FaissIndex,
        n: idx_t,
        x: *const f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Same as add, but stores xids instead of sequential ids.
    ///
    /// The default implementation fails with an assertion, as it is
    /// not supported by all indexes.
    ///
    /// @param index  opaque pointer to index object
    /// @param xids   if non-null, ids to store for the vectors (size n)
    pub fn faiss_Index_add_with_ids(
        index: *mut FaissIndex,
        n: idx_t,
        x: *const f32,
        xids: *const ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// query n vectors of dimension d to the index.
    ///
    /// return at most k vectors. If there are not enough results for a
    /// query, the result array is padded with -1s.
    ///
    /// @param index       opaque pointer to index object
    /// @param x           input vectors to search, size n * d
    /// @param labels      output labels of the NNs, size n*k
    /// @param distances   output pairwise distances, size n*k
    pub fn faiss_Index_search(
        index: *const FaissIndex,
        n: idx_t,
        x: *const f32,
        k: idx_t,
        distances: *mut f32,
        labels: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// query n vectors of dimension d to the index.
    ///
    /// return all vectors with distance < radius. Note that many
    /// indexes do not implement the range_search (only the k-NN search
    /// is mandatory).
    ///
    /// @param index       opaque pointer to index object
    /// @param x           input vectors to search, size n * d
    /// @param radius      search radius
    /// @param result      result table
    pub fn faiss_Index_range_search(
        index: *const FaissIndex,
        n: idx_t,
        x: *const f32,
        radius: f32,
        result: *mut FaissRangeSearchResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// return the indexes of the k vectors closest to the query x.
    ///
    /// This function is identical as search but only return labels of neighbors.
    /// @param index       opaque pointer to index object
    /// @param x           input vectors to search, size n * d
    /// @param labels      output labels of the NNs, size n*k
    pub fn faiss_Index_assign(
        index: *mut FaissIndex,
        n: idx_t,
        x: *const f32,
        labels: *mut idx_t,
        k: idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// removes all elements from the database.
    /// @param index       opaque pointer to index object
    pub fn faiss_Index_reset(index: *mut FaissIndex) -> ::std::os::raw::c_int;
}
extern "C" {
    /// removes IDs from the index. Not supported by all indexes
    /// @param index       opaque pointer to index object
    /// @param nremove     output for the number of IDs removed
    pub fn faiss_Index_remove_ids(
        index: *mut FaissIndex,
        sel: *const FaissIDSelector,
        n_removed: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Reconstruct a stored vector (or an approximation if lossy coding)
    ///
    /// this function may not be defined for some indexes
    /// @param index       opaque pointer to index object
    /// @param key         id of the vector to reconstruct
    /// @param recons      reconstucted vector (size d)
    pub fn faiss_Index_reconstruct(
        index: *const FaissIndex,
        key: idx_t,
        recons: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Reconstruct vectors i0 to i0 + ni - 1
    ///
    /// this function may not be defined for some indexes
    /// @param index       opaque pointer to index object
    /// @param recons      reconstucted vector (size ni * d)
    pub fn faiss_Index_reconstruct_n(
        index: *const FaissIndex,
        i0: idx_t,
        ni: idx_t,
        recons: *mut f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Computes a residual vector after indexing encoding.
    ///
    /// The residual vector is the difference between a vector and the
    /// reconstruction that can be decoded from its representation in
    /// the index. The residual can be used for multiple-stage indexing
    /// methods, like IndexIVF's methods.
    ///
    /// @param index       opaque pointer to index object
    /// @param x           input vector, size d
    /// @param residual    output residual vector, size d
    /// @param key         encoded index, as returned by search and assign
    pub fn faiss_Index_compute_residual(
        index: *const FaissIndex,
        x: *const f32,
        residual: *mut f32,
        key: idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Display the actual class name and some more info
    /// @param index       opaque pointer to index object
    pub fn faiss_Index_display(index: *const FaissIndex) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Build and index with the sequence of processing steps described in
    /// the string.
    pub fn faiss_index_factory(
        p_index: *mut *mut FaissIndex,
        d: ::std::os::raw::c_int,
        description: *const ::std::os::raw::c_char,
        metric: FaissMetricType,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissParameterRange_H {
    _unused: [u8; 0],
}
pub type FaissParameterRange = FaissParameterRange_H;
extern "C" {
    pub fn faiss_ParameterRange_name(
        arg1: *const FaissParameterRange,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    /// Getter for the values in the range. The output values are invalidated
    /// upon any other modification of the range.
    pub fn faiss_ParameterRange_values(
        arg1: *mut FaissParameterRange,
        arg2: *mut *mut f64,
        arg3: *mut usize,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissParameterSpace_H {
    _unused: [u8; 0],
}
pub type FaissParameterSpace = FaissParameterSpace_H;
extern "C" {
    /// Parameter space default constructor
    pub fn faiss_ParameterSpace_new(space: *mut *mut FaissParameterSpace) -> ::std::os::raw::c_int;
}
extern "C" {
    /// nb of combinations, = product of values sizes
    pub fn faiss_ParameterSpace_n_combinations(arg1: *const FaissParameterSpace) -> usize;
}
extern "C" {
    /// get string representation of the combination
    /// by writing it to the given character buffer.
    /// A buffer size of 1000 ensures that the full name is collected.
    pub fn faiss_ParameterSpace_combination_name(
        arg1: *const FaissParameterSpace,
        arg2: usize,
        arg3: *mut ::std::os::raw::c_char,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// set a combination of parameters described by a string
    pub fn faiss_ParameterSpace_set_index_parameters(
        arg1: *const FaissParameterSpace,
        arg2: *mut FaissIndex,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// set a combination of parameters on an index
    pub fn faiss_ParameterSpace_set_index_parameters_cno(
        arg1: *const FaissParameterSpace,
        arg2: *mut FaissIndex,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// set one of the parameters
    pub fn faiss_ParameterSpace_set_index_parameter(
        arg1: *const FaissParameterSpace,
        arg2: *mut FaissIndex,
        arg3: *const ::std::os::raw::c_char,
        arg4: f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// print a description on stdout
    pub fn faiss_ParameterSpace_display(arg1: *const FaissParameterSpace);
}
extern "C" {
    /// add a new parameter (or return it if it exists)
    pub fn faiss_ParameterSpace_add_range(
        arg1: *mut FaissParameterSpace,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut FaissParameterRange,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_RangeSearchResult_nq(arg1: *const FaissRangeSearchResult) -> usize;
}
extern "C" {
    pub fn faiss_RangeSearchResult_new(
        p_rsr: *mut *mut FaissRangeSearchResult,
        nq: idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_RangeSearchResult_new_with(
        p_rsr: *mut *mut FaissRangeSearchResult,
        nq: idx_t,
        alloc_lims: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// called when lims contains the nb of elements result entries
    /// for each query
    pub fn faiss_RangeSearchResult_do_allocation(
        rsr: *mut FaissRangeSearchResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_RangeSearchResult_free(obj: *mut FaissRangeSearchResult);
}
extern "C" {
    pub fn faiss_RangeSearchResult_buffer_size(arg1: *const FaissRangeSearchResult) -> usize;
}
extern "C" {
    /// getter for lims: size (nq + 1)
    pub fn faiss_RangeSearchResult_lims(rsr: *mut FaissRangeSearchResult, lims: *mut *mut usize);
}
extern "C" {
    /// getter for labels and respective distances (not sorted):
    /// result for query i is labels[lims[i]:lims[i+1]]
    pub fn faiss_RangeSearchResult_labels(
        rsr: *mut FaissRangeSearchResult,
        labels: *mut *mut idx_t,
        distances: *mut *mut f32,
    );
}
extern "C" {
    pub fn faiss_IDSelector_free(obj: *mut FaissIDSelector);
}
extern "C" {
    /// Encapsulates a set of ids to remove.
    pub fn faiss_IDSelector_is_member(
        sel: *const FaissIDSelector,
        id: idx_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissIDSelectorRange_H {
    _unused: [u8; 0],
}
pub type FaissIDSelectorRange = FaissIDSelectorRange_H;
extern "C" {
    pub fn faiss_IDSelectorRange_free(obj: *mut FaissIDSelectorRange);
}
extern "C" {
    pub fn faiss_IDSelectorRange_imin(arg1: *const FaissIDSelectorRange) -> idx_t;
}
extern "C" {
    pub fn faiss_IDSelectorRange_imax(arg1: *const FaissIDSelectorRange) -> idx_t;
}
extern "C" {
    /// remove ids between [imni, imax)
    pub fn faiss_IDSelectorRange_new(
        p_sel: *mut *mut FaissIDSelectorRange,
        imin: idx_t,
        imax: idx_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissIDSelectorBatch_H {
    _unused: [u8; 0],
}
pub type FaissIDSelectorBatch = FaissIDSelectorBatch_H;
extern "C" {
    pub fn faiss_IDSelectorBatch_nbits(arg1: *const FaissIDSelectorBatch) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IDSelectorBatch_mask(arg1: *const FaissIDSelectorBatch) -> idx_t;
}
extern "C" {
    /// Remove ids from a set. Repetitions of ids in the indices set
    /// passed to the constructor does not hurt performance. The hash
    /// function used for the bloom filter and GCC's implementation of
    /// unordered_set are just the least significant bits of the id. This
    /// works fine for random ids or ids in sequences but will produce many
    /// hash collisions if lsb's are always the same
    pub fn faiss_IDSelectorBatch_new(
        p_sel: *mut *mut FaissIDSelectorBatch,
        n: ::std::os::raw::c_long,
        indices: *const idx_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissBufferList_H {
    _unused: [u8; 0],
}
pub type FaissBufferList = FaissBufferList_H;
extern "C" {
    pub fn faiss_BufferList_free(obj: *mut FaissBufferList);
}
extern "C" {
    pub fn faiss_BufferList_buffer_size(arg1: *const FaissBufferList) -> usize;
}
extern "C" {
    pub fn faiss_BufferList_wp(arg1: *const FaissBufferList) -> usize;
}
/// List of temporary buffers used to store results before they are
/// copied to the RangeSearchResult object.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissBuffer {
    pub ids: *mut idx_t,
    pub dis: *mut f32,
}
#[test]
fn bindgen_test_layout_FaissBuffer() {
    assert_eq!(
        ::std::mem::size_of::<FaissBuffer>(),
        16usize,
        concat!("Size of: ", stringify!(FaissBuffer))
    );
    assert_eq!(
        ::std::mem::align_of::<FaissBuffer>(),
        8usize,
        concat!("Alignment of ", stringify!(FaissBuffer))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissBuffer>())).ids as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissBuffer),
            "::",
            stringify!(ids)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissBuffer>())).dis as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissBuffer),
            "::",
            stringify!(dis)
        )
    );
}
extern "C" {
    pub fn faiss_BufferList_append_buffer(bl: *mut FaissBufferList) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_BufferList_new(
        p_bl: *mut *mut FaissBufferList,
        buffer_size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_BufferList_add(
        bl: *mut FaissBufferList,
        id: idx_t,
        dis: f32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// copy elemnts ofs:ofs+n-1 seen as linear data in the buffers to
    /// tables dest_ids, dest_dis
    pub fn faiss_BufferList_copy_range(
        bl: *mut FaissBufferList,
        ofs: usize,
        n: usize,
        dest_ids: *mut idx_t,
        dest_dis: *mut f32,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissRangeSearchPartialResult_H {
    _unused: [u8; 0],
}
pub type FaissRangeSearchPartialResult = FaissRangeSearchPartialResult_H;
extern "C" {
    pub fn faiss_RangeSearchPartialResult_res(
        arg1: *const FaissRangeSearchPartialResult,
    ) -> *mut FaissRangeSearchResult;
}
extern "C" {
    /// the entries in the buffers are split per query
    pub fn faiss_RangeSearchPartialResult_new(
        p_res: *mut *mut FaissRangeSearchPartialResult,
        res_in: *mut FaissRangeSearchResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_RangeSearchPartialResult_finalize(
        res: *mut FaissRangeSearchPartialResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// called by range_search before do_allocation
    pub fn faiss_RangeSearchPartialResult_set_lims(
        res: *mut FaissRangeSearchPartialResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// called by range_search after do_allocation
    pub fn faiss_RangeSearchPartialResult_set_result(
        res: *mut FaissRangeSearchPartialResult,
        incremental: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissQueryResult_H {
    _unused: [u8; 0],
}
pub type FaissQueryResult = FaissQueryResult_H;
extern "C" {
    pub fn faiss_QueryResult_qno(arg1: *const FaissQueryResult) -> idx_t;
}
extern "C" {
    pub fn faiss_QueryResult_nres(arg1: *const FaissQueryResult) -> usize;
}
extern "C" {
    pub fn faiss_QueryResult_pres(
        arg1: *const FaissQueryResult,
    ) -> *mut FaissRangeSearchPartialResult;
}
extern "C" {
    /// result structure for a single query
    pub fn faiss_RangeSearchPartialResult_new_result(
        res: *mut FaissRangeSearchPartialResult,
        qno: idx_t,
        qr: *mut *mut FaissQueryResult,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_QueryResult_add(
        qr: *mut FaissQueryResult,
        dis: f32,
        id: idx_t,
    ) -> ::std::os::raw::c_int;
}
/// Class for the clustering parameters. Can be passed to the
/// constructor of the Clustering object.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissClusteringParameters {
    /// < clustering iterations
    pub niter: ::std::os::raw::c_int,
    /// < redo clustering this many times and keep best
    pub nredo: ::std::os::raw::c_int,
    /// < (bool)
    pub verbose: ::std::os::raw::c_int,
    /// < (bool) do we want normalized centroids?
    pub spherical: ::std::os::raw::c_int,
    /// < (bool) update index after each iteration?
    pub update_index: ::std::os::raw::c_int,
    /// < (bool) use the centroids provided as input and do not change them during iterations
    pub frozen_centroids: ::std::os::raw::c_int,
    /// < otherwise you get a warning
    pub min_points_per_centroid: ::std::os::raw::c_int,
    /// < to limit size of dataset
    pub max_points_per_centroid: ::std::os::raw::c_int,
    /// < seed for the random number generator
    pub seed: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_FaissClusteringParameters() {
    assert_eq!(
        ::std::mem::size_of::<FaissClusteringParameters>(),
        36usize,
        concat!("Size of: ", stringify!(FaissClusteringParameters))
    );
    assert_eq!(
        ::std::mem::align_of::<FaissClusteringParameters>(),
        4usize,
        concat!("Alignment of ", stringify!(FaissClusteringParameters))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissClusteringParameters>())).niter as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(niter)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissClusteringParameters>())).nredo as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(nredo)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FaissClusteringParameters>())).verbose as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(verbose)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FaissClusteringParameters>())).spherical as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(spherical)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FaissClusteringParameters>())).update_index as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(update_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FaissClusteringParameters>())).frozen_centroids as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(frozen_centroids)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FaissClusteringParameters>())).min_points_per_centroid
                as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(min_points_per_centroid)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FaissClusteringParameters>())).max_points_per_centroid
                as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(max_points_per_centroid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissClusteringParameters>())).seed as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissClusteringParameters),
            "::",
            stringify!(seed)
        )
    );
}
extern "C" {
    /// Sets the ClusteringParameters object with reasonable defaults
    pub fn faiss_ClusteringParameters_init(params: *mut FaissClusteringParameters);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissClustering_H {
    _unused: [u8; 0],
}
pub type FaissClustering = FaissClustering_H;
extern "C" {
    pub fn faiss_Clustering_niter(arg1: *const FaissClustering) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_nredo(arg1: *const FaissClustering) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_verbose(arg1: *const FaissClustering) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_spherical(arg1: *const FaissClustering) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_update_index(arg1: *const FaissClustering) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_frozen_centroids(arg1: *const FaissClustering)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_min_points_per_centroid(
        arg1: *const FaissClustering,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_max_points_per_centroid(
        arg1: *const FaissClustering,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_seed(arg1: *const FaissClustering) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_d(arg1: *const FaissClustering) -> usize;
}
extern "C" {
    pub fn faiss_Clustering_k(arg1: *const FaissClustering) -> usize;
}
extern "C" {
    /// getter for centroids (size = k * d)
    pub fn faiss_Clustering_centroids(
        clustering: *mut FaissClustering,
        centroids: *mut *mut f32,
        size: *mut usize,
    );
}
extern "C" {
    /// getter for objective values (sum of distances reported by index)
    /// over iterations
    pub fn faiss_Clustering_obj(
        clustering: *mut FaissClustering,
        obj: *mut *mut f32,
        size: *mut usize,
    );
}
extern "C" {
    /// the only mandatory parameters are k and d
    pub fn faiss_Clustering_new(
        p_clustering: *mut *mut FaissClustering,
        d: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_new_with_params(
        p_clustering: *mut *mut FaissClustering,
        d: ::std::os::raw::c_int,
        k: ::std::os::raw::c_int,
        cp: *const FaissClusteringParameters,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_train(
        clustering: *mut FaissClustering,
        n: idx_t,
        x: *const f32,
        index: *mut FaissIndex,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_Clustering_free(clustering: *mut FaissClustering);
}
extern "C" {
    /// simplified interface
    ///
    /// @param d dimension of the data
    /// @param n nb of training vectors
    /// @param k nb of output centroids
    /// @param x training set (size n * d)
    /// @param centroids output centroids (size k * d)
    /// @param q_error final quantization error
    /// @return error code
    pub fn faiss_kmeans_clustering(
        d: usize,
        n: usize,
        k: usize,
        x: *const f32,
        centroids: *mut f32,
        q_error: *mut f32,
    ) -> ::std::os::raw::c_int;
}
pub const FaissErrorCode_OK: FaissErrorCode = 0;
pub const FaissErrorCode_UNKNOWN_EXCEPT: FaissErrorCode = -1;
pub const FaissErrorCode_FAISS_EXCEPT: FaissErrorCode = -2;
pub const FaissErrorCode_STD_EXCEPT: FaissErrorCode = -4;
pub type FaissErrorCode = i32;
extern "C" {
    /// Get the error message of the last failed operation performed by Faiss.
    /// The given pointer is only invalid until another Faiss function is
    /// called.
    pub fn faiss_get_last_error() -> *const ::std::os::raw::c_char;
}
pub type FaissIndexFlat = FaissIndex_H;
extern "C" {
    /// Opaque type for IndexFlat
    pub fn faiss_IndexFlat_new(p_index: *mut *mut FaissIndexFlat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexFlat_new_with(
        p_index: *mut *mut FaissIndexFlat,
        d: idx_t,
        metric: FaissMetricType,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// get a pointer to the index's internal data (the `xb` field). The outputs
    /// become invalid after any data addition or removal operation.
    ///
    /// @param index   opaque pointer to index object
    /// @param p_xb    output, the pointer to the beginning of `xb`.
    /// @param p_size  output, the current size of `sb` in number of float values.
    pub fn faiss_IndexFlat_xb(index: *mut FaissIndexFlat, p_xb: *mut *mut f32, p_size: *mut usize);
}
extern "C" {
    /// attempt a dynamic cast to a flat index, thus checking
    /// check whether the underlying index type is `IndexFlat`.
    ///
    /// @param index opaque pointer to index object
    /// @return the same pointer if the index is a flat index, NULL otherwise
    pub fn faiss_IndexFlat_cast(index: *mut FaissIndex) -> *mut FaissIndexFlat;
}
extern "C" {
    pub fn faiss_IndexFlat_free(obj: *mut FaissIndexFlat);
}
extern "C" {
    /// compute distance with a subset of vectors
    ///
    /// @param index   opaque pointer to index object
    /// @param x       query vectors, size n * d
    /// @param labels  indices of the vectors that should be compared
    /// for each query vector, size n * k
    /// @param distances
    /// corresponding output distances, size n * k
    pub fn faiss_IndexFlat_compute_distance_subset(
        index: *mut FaissIndex,
        n: idx_t,
        x: *const f32,
        k: idx_t,
        distances: *mut f32,
        labels: *const idx_t,
    ) -> ::std::os::raw::c_int;
}
pub type FaissIndexFlatIP = FaissIndex_H;
extern "C" {
    /// Opaque type for IndexFlatIP
    pub fn faiss_IndexFlatIP_new(p_index: *mut *mut FaissIndexFlatIP) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexFlatIP_new_with(
        p_index: *mut *mut FaissIndexFlatIP,
        d: idx_t,
    ) -> ::std::os::raw::c_int;
}
pub type FaissIndexFlatL2 = FaissIndex_H;
extern "C" {
    /// Opaque type for IndexFlatL2
    pub fn faiss_IndexFlatL2_new(p_index: *mut *mut FaissIndexFlatL2) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexFlatL2_new_with(
        p_index: *mut *mut FaissIndexFlatL2,
        d: idx_t,
    ) -> ::std::os::raw::c_int;
}
pub type FaissIndexFlatL2BaseShift = FaissIndex_H;
extern "C" {
    /// Opaque type for IndexFlatL2BaseShift
    ///
    /// same as an IndexFlatL2 but a value is subtracted from each distance
    pub fn faiss_IndexFlatL2BaseShift_new(
        p_index: *mut *mut FaissIndexFlatL2BaseShift,
        d: idx_t,
        nshift: usize,
        shift: *const f32,
    ) -> ::std::os::raw::c_int;
}
pub type FaissIndexRefineFlat = FaissIndex_H;
extern "C" {
    /// Opaque type for IndexRefineFlat
    ///
    /// Index that queries in a base_index (a fast one) and refines the
    /// results with an exact search, hopefully improving the results.
    pub fn faiss_IndexRefineFlat_new(
        p_index: *mut *mut FaissIndexRefineFlat,
        base_index: *mut FaissIndex,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexRefineFlat_free(obj: *mut FaissIndexRefineFlat);
}
pub type FaissIndexFlat1D = FaissIndex_H;
extern "C" {
    /// Opaque type for IndexFlat1D
    ///
    /// optimized version for 1D "vectors"
    pub fn faiss_IndexFlat1D_new(p_index: *mut *mut FaissIndexFlat1D) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexFlat1D_new_with(
        p_index: *mut *mut FaissIndexFlat1D,
        continuous_update: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexFlat1D_update_permutation(
        index: *mut FaissIndexFlat1D,
    ) -> ::std::os::raw::c_int;
}
pub type FaissIndexIVF = FaissIndex_H;
extern "C" {
    pub fn faiss_IndexIVF_free(obj: *mut FaissIndexIVF);
}
extern "C" {
    pub fn faiss_IndexIVF_nlist(arg1: *const FaissIndexIVF) -> usize;
}
extern "C" {
    pub fn faiss_IndexIVF_nprobe(arg1: *const FaissIndexIVF) -> usize;
}
extern "C" {
    pub fn faiss_IndexIVF_quantizer(arg1: *const FaissIndexIVF) -> *mut FaissIndex;
}
extern "C" {
    pub fn faiss_IndexIVF_quantizer_trains_alone(
        arg1: *const FaissIndexIVF,
    ) -> ::std::os::raw::c_char;
}
extern "C" {
    pub fn faiss_IndexIVF_own_fields(arg1: *const FaissIndexIVF) -> ::std::os::raw::c_int;
}
extern "C" {
    /// moves the entries from another dataset to self. On output,
    /// other is empty. add_id is added to all moved ids (for
    /// sequential ids, this would be this->ntotal
    pub fn faiss_IndexIVF_merge_from(
        index: *mut FaissIndexIVF,
        other: *mut FaissIndexIVF,
        add_id: idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// copy a subset of the entries index to the other index
    ///
    /// if subset_type == 0: copies ids in [a1, a2)
    /// if subset_type == 1: copies ids if id % a1 == a2
    /// if subset_type == 2: copies inverted lists such that a1
    /// elements are left before and a2 elements are after
    pub fn faiss_IndexIVF_copy_subset_to(
        index: *const FaissIndexIVF,
        other: *mut FaissIndexIVF,
        subset_type: ::std::os::raw::c_int,
        a1: ::std::os::raw::c_long,
        a2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// search a set of vectors, that are pre-quantized by the IVF
    /// quantizer. Fill in the corresponding heaps with the query
    /// results. search() calls this.
    ///
    /// @param n      nb of vectors to query
    /// @param x      query vectors, size nx * d
    /// @param assign coarse quantization indices, size nx * nprobe
    /// @param centroid_dis
    /// distances to coarse centroids, size nx * nprobe
    /// @param distance
    /// output distances, size n * k
    /// @param labels output labels, size n * k
    /// @param store_pairs store inv list index + inv list offset
    /// instead in upper/lower 32 bit of result,
    /// instead of ids (used for reranking).
    pub fn faiss_IndexIVF_search_preassigned(
        index: *const FaissIndexIVF,
        n: idx_t,
        x: *const f32,
        k: idx_t,
        assign: *const idx_t,
        centroid_dis: *const f32,
        distances: *mut f32,
        labels: *mut idx_t,
        store_pairs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexIVF_get_list_size(index: *const FaissIndexIVF, list_no: usize) -> usize;
}
extern "C" {
    /// intialize a direct map
    ///
    /// @param new_maintain_direct_map    if true, create a direct map,
    /// else clear it
    pub fn faiss_IndexIVF_make_direct_map(
        index: *mut FaissIndexIVF,
        new_maintain_direct_map: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// 1= perfectly balanced, >1: imbalanced
    pub fn faiss_IndexIVF_imbalance_factor(index: *const FaissIndexIVF) -> f64;
}
extern "C" {
    /// display some stats about the inverted lists
    pub fn faiss_IndexIVF_print_stats(index: *const FaissIndexIVF);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissIndexIVFStats {
    pub nq: usize,
    pub nlist: usize,
    pub ndis: usize,
}
#[test]
fn bindgen_test_layout_FaissIndexIVFStats() {
    assert_eq!(
        ::std::mem::size_of::<FaissIndexIVFStats>(),
        24usize,
        concat!("Size of: ", stringify!(FaissIndexIVFStats))
    );
    assert_eq!(
        ::std::mem::align_of::<FaissIndexIVFStats>(),
        8usize,
        concat!("Alignment of ", stringify!(FaissIndexIVFStats))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissIndexIVFStats>())).nq as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissIndexIVFStats),
            "::",
            stringify!(nq)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissIndexIVFStats>())).nlist as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissIndexIVFStats),
            "::",
            stringify!(nlist)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FaissIndexIVFStats>())).ndis as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FaissIndexIVFStats),
            "::",
            stringify!(ndis)
        )
    );
}
extern "C" {
    pub fn faiss_IndexIVFStats_reset(stats: *mut FaissIndexIVFStats);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FaissIndexIVFFlat_H {
    _unused: [u8; 0],
}
pub type FaissIndexIVFFlat = FaissIndexIVFFlat_H;
extern "C" {
    pub fn faiss_IndexIVFFlat_free(obj: *mut FaissIndexIVFFlat);
}
extern "C" {
    /// Inverted file with stored vectors. Here the inverted file
    /// pre-selects the vectors to be searched, but they are not otherwise
    /// encoded, the code array just contains the raw float entries.
    pub fn faiss_IndexIVFFlat_new(p_index: *mut *mut FaissIndexIVFFlat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexIVFFlat_new_with(
        p_index: *mut *mut FaissIndexIVFFlat,
        quantizer: *mut FaissIndex,
        d: usize,
        nlist: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexIVFFlat_new_with_metric(
        p_index: *mut *mut FaissIndexIVFFlat,
        quantizer: *mut FaissIndex,
        d: usize,
        nlist: usize,
        metric: FaissMetricType,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn faiss_IndexIVFFlat_add_core(
        index: *mut FaissIndexIVFFlat,
        n: idx_t,
        x: *const f32,
        xids: *const ::std::os::raw::c_long,
        precomputed_idx: *const ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    /// Update a subset of vectors.
    ///
    /// The index must have a direct_map
    ///
    /// @param nv     nb of vectors to update
    /// @param idx    vector indices to update, size nv
    /// @param v      vectors of new values, size nv*d
    pub fn faiss_IndexIVFFlat_update_vectors(
        index: *mut FaissIndexIVFFlat,
        nv: ::std::os::raw::c_int,
        idx: *mut idx_t,
        v: *const f32,
    ) -> ::std::os::raw::c_int;
}
