<script setup lang="ts">
import type { CreerFaculteForm, AdminEcolePartenaire } from '~/types/admin'

definePageMeta({ layout: 'admin', middleware: ['admin'] })

const { creer, loading, error } = useAdminFacultes()
const { listerToutes } = useAdminEcolesPartenaires()
const router = useRouter()

const ecoles = ref<AdminEcolePartenaire[]>([])

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
})

const erreurLocale = ref<string | null>(null)

const soumettre = async () => {
  erreurLocale.value = null
  if (!form.titre.trim()) { erreurLocale.value = 'Le titre est requis'; return }
  if (!form.acronyme.trim()) { erreurLocale.value = "L'acronyme est requis"; return }
  if (!form.description.trim()) { erreurLocale.value = 'La description est requise'; return }
  if (!form.ecole_partenaire_id) { erreurLocale.value = "L'école partenaire est requise"; return }
  try {
    await creer({ ...form })
    router.push('/admin/facultes')
  } catch (e: any) {
    erreurLocale.value = e?.data?.error || e?.message || 'Erreur lors de la création'
  }
}

onMounted(async () => {
  ecoles.value = (await listerToutes()).filter(e => e.actif)
})
</script>

<template>
  <div>
    <AdminPageHeader titre="Nouvelle faculté" sous-titre="Ajouter une faculté partenaire">
      <template #actions>
        <NuxtLink to="/admin/facultes" class="btn btn-ghost btn-sm">
          <font-awesome-icon icon="arrow-left" class="mr-1" /> Retour
        </NuxtLink>
      </template>
    </AdminPageHeader>

    <div v-if="erreurLocale || error" class="alert alert-error mb-4">
      <font-awesome-icon icon="circle-exclamation" />
      <span>{{ erreurLocale || error }}</span>
    </div>

    <form @submit.prevent="soumettre">
      <AdminFaculteForm v-model="form" :ecoles="ecoles" mode="create" />

      <div class="flex justify-end gap-2 pt-6">
        <NuxtLink to="/admin/facultes" class="btn btn-ghost">Annuler</NuxtLink>
        <button type="submit" class="btn btn-primary" :class="{ loading }" :disabled="loading">
          <font-awesome-icon v-if="!loading" icon="plus" class="mr-1" /> Créer
        </button>
      </div>
    </form>
  </div>
</template>
