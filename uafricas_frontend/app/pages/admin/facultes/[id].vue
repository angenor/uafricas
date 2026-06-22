<script setup lang="ts">
import type { CreerFaculteForm, AdminEcolePartenaire } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const route = useRoute()
const id = route.params.id as string

const { faculteDetail, loading, error, chargerDetail, modifier } = useAdminFacultes()
const { listerToutes } = useAdminEcolesPartenaires()

const ecoles = ref<AdminEcolePartenaire[]>([])
const saving = ref(false)
const erreurLocale = ref<string | null>(null)
const successMsg = ref<string | null>(null)

const form = reactive<CreerFaculteForm>({
  titre: '',
  acronyme: '',
  description: '',
  image_couverture_url: '',
  logo_url: '',
  ecole_partenaire_id: '',
  domaines_etudes: [],
  programmes_licence: [],
  programmes_master: [],
  programmes_doctorat: [],
  programmes_certificats: [],
  diplome_minimum: '',
  langues_enseignement: [],
  frais_scolarite_min: null,
  frais_scolarite_max: null,
  bourses_possibles: false,
  periodes_inscription: '',
  points_forts: [],
  accepte_nouveaux_inscrits: true,
  statut: 'active',
  referent_id: null,
  nombre_inscrits_total: 0,
  nombre_inscrits_annee: 0,
})

const charger = async () => {
  await chargerDetail(id)
  const f = faculteDetail.value
  if (f) {
    form.titre = f.titre
    form.acronyme = f.acronyme
    form.description = f.description
    form.image_couverture_url = f.image_couverture_url || ''
    form.logo_url = f.logo_url || ''
    form.ecole_partenaire_id = f.ecole_partenaire_id
    form.domaines_etudes = [...f.domaines_etudes]
    form.programmes_licence = [...f.programmes_licence]
    form.programmes_master = [...f.programmes_master]
    form.programmes_doctorat = [...f.programmes_doctorat]
    form.programmes_certificats = [...f.programmes_certificats]
    form.diplome_minimum = f.diplome_minimum || ''
    form.langues_enseignement = [...f.langues_enseignement]
    form.frais_scolarite_min = f.frais_scolarite_min
    form.frais_scolarite_max = f.frais_scolarite_max
    form.bourses_possibles = f.bourses_possibles
    form.periodes_inscription = f.periodes_inscription || ''
    form.points_forts = [...f.points_forts]
    form.accepte_nouveaux_inscrits = f.accepte_nouveaux_inscrits
    form.statut = f.statut
    form.referent_id = f.referent_id
    form.nombre_inscrits_total = f.nombre_inscrits_total
    form.nombre_inscrits_annee = f.nombre_inscrits_annee
  }
}

const sauvegarder = async () => {
  erreurLocale.value = null
  successMsg.value = null
  if (!form.titre.trim()) { erreurLocale.value = 'Le titre est requis'; return }
  if (!form.acronyme.trim()) { erreurLocale.value = "L'acronyme est requis"; return }
  if (!form.description.trim()) { erreurLocale.value = 'La description est requise'; return }
  if (!form.ecole_partenaire_id) { erreurLocale.value = "L'école partenaire est requise"; return }
  saving.value = true
  try {
    await modifier(id, { ...form })
    successMsg.value = 'Faculté mise à jour'
    setTimeout(() => { successMsg.value = null }, 3000)
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

onMounted(async () => {
  ecoles.value = await listerToutes()
  await charger()
})
</script>

<template>
  <div>
    <AdminPageHeader :titre="faculteDetail?.titre || 'Chargement...'" sous-titre="Édition de la faculté">
      <template #actions>
        <NuxtLink to="/admin/facultes" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="loading && !faculteDetail" class="flex justify-center py-12">
      <span class="loading loading-spinner loading-lg" />
    </div>

    <template v-else-if="faculteDetail">
      <div v-if="erreurLocale || error" class="alert alert-error mb-4">
        <font-awesome-icon icon="circle-exclamation" />
        <span>{{ erreurLocale || error }}</span>
        <button class="btn btn-ghost btn-xs" @click="erreurLocale = null">
          <font-awesome-icon icon="xmark" />
        </button>
      </div>
      <div v-if="successMsg" class="alert alert-success mb-4">
        <font-awesome-icon icon="circle-check" />
        <span>{{ successMsg }}</span>
      </div>

      <form @submit.prevent="sauvegarder">
        <AdminFaculteForm v-model="form" :ecoles="ecoles" mode="edit" />

        <div class="flex items-center justify-between pt-6">
          <div class="text-sm text-base-content/50">ID : {{ faculteDetail.id.substring(0, 8) }}…</div>
          <button type="submit" class="btn btn-primary" :class="{ loading: saving }" :disabled="saving">
            <font-awesome-icon v-if="!saving" icon="floppy-disk" class="mr-1" /> Enregistrer
          </button>
        </div>
      </form>
    </template>
  </div>
</template>
