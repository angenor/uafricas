<script setup lang="ts">
import type { CreerFaculteForm, AdminEcolePartenaire } from '~/types/admin'

const props = defineProps<{
  modelValue: CreerFaculteForm
  ecoles: AdminEcolePartenaire[]
  mode: 'create' | 'edit'
}>()

// Le form est un objet réactif partagé : on le mute directement.
const form = props.modelValue

/** Fabrique un proxy textarea (1 élément par ligne) sur un champ tableau du form. */
function champListe(cle: keyof CreerFaculteForm) {
  return computed<string>({
    get: () => ((form[cle] as string[]) || []).join('\n'),
    set: (val: string) => {
      ;(form[cle] as unknown as string[]) = val
        .split('\n')
        .map(s => s.trim())
        .filter(Boolean)
    },
  })
}

const domaines = champListe('domaines_etudes')
const licence = champListe('programmes_licence')
const master = champListe('programmes_master')
const doctorat = champListe('programmes_doctorat')
const certificats = champListe('programmes_certificats')
const langues = champListe('langues_enseignement')
const pointsForts = champListe('points_forts')
</script>

<template>
  <div class="space-y-6">
    <!-- Identité -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="font-semibold mb-2">Identité</h3>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="form-control md:col-span-2">
            <label class="label"><span class="label-text">Titre de la faculté *</span></label>
            <input v-model="form.titre" type="text" class="input input-bordered" required>
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Acronyme *</span></label>
            <input v-model="form.acronyme" type="text" class="input input-bordered" required>
          </div>
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text">École partenaire *</span></label>
          <select v-model="form.ecole_partenaire_id" class="select select-bordered" required>
            <option value="" disabled>Sélectionner une école partenaire</option>
            <option v-for="e in ecoles" :key="e.id" :value="e.id">
              {{ e.nom }}{{ e.ville ? ` — ${e.ville}` : '' }}
            </option>
          </select>
          <label class="label">
            <span class="label-text-alt">
              Aucune école ?
              <NuxtLink to="/admin/ecoles-partenaires/create" class="link link-primary">En créer une</NuxtLink>
            </span>
          </label>
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text">Description *</span></label>
          <textarea v-model="form.description" class="textarea textarea-bordered" rows="4" required />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Image illustrative (couverture)</span></label>
            <OpportuniteAfriqueImageUploadField v-model="form.image_couverture_url" label="" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Logo</span></label>
            <OpportuniteAfriqueImageUploadField v-model="form.logo_url" label="" />
          </div>
        </div>
      </div>
    </div>

    <!-- Domaines & programmes -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="font-semibold mb-2">Domaines & programmes</h3>
        <p class="text-sm text-base-content/50 mb-2">Un élément par ligne.</p>

        <div class="form-control">
          <label class="label"><span class="label-text">Domaines d'études</span></label>
          <textarea v-model="domaines" class="textarea textarea-bordered" rows="3" placeholder="Économie&#10;Gestion&#10;..." />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Programmes Licence</span></label>
            <textarea v-model="licence" class="textarea textarea-bordered" rows="3" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Programmes Master</span></label>
            <textarea v-model="master" class="textarea textarea-bordered" rows="3" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Programmes Doctorat</span></label>
            <textarea v-model="doctorat" class="textarea textarea-bordered" rows="3" />
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Certificats</span></label>
            <textarea v-model="certificats" class="textarea textarea-bordered" rows="3" />
          </div>
        </div>
      </div>
    </div>

    <!-- Conditions d'admission -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="font-semibold mb-2">Conditions d'admission</h3>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Diplôme minimum</span></label>
            <input v-model="form.diplome_minimum" type="text" class="input input-bordered" placeholder="Baccalauréat...">
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Périodes d'inscription</span></label>
            <input v-model="form.periodes_inscription" type="text" class="input input-bordered" placeholder="Septembre - Octobre">
          </div>
        </div>

        <div class="form-control">
          <label class="label"><span class="label-text">Langues d'enseignement</span></label>
          <textarea v-model="langues" class="textarea textarea-bordered" rows="2" placeholder="Français&#10;Anglais" />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Frais scolarité min (FCFA)</span></label>
            <input v-model.number="form.frais_scolarite_min" type="number" min="0" class="input input-bordered">
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Frais scolarité max (FCFA)</span></label>
            <input v-model.number="form.frais_scolarite_max" type="number" min="0" class="input input-bordered">
          </div>
          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3 mt-8">
              <input v-model="form.bourses_possibles" type="checkbox" class="toggle toggle-success" />
              <span class="label-text">Bourses possibles</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- Points forts & statut -->
    <div class="card bg-base-100 shadow-sm">
      <div class="card-body">
        <h3 class="font-semibold mb-2">Points forts & statut</h3>

        <div class="form-control">
          <label class="label"><span class="label-text">Points forts</span></label>
          <textarea v-model="pointsForts" class="textarea textarea-bordered" rows="3" placeholder="Un point fort par ligne" />
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Statut</span></label>
            <select v-model="form.statut" class="select select-bordered">
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
            </select>
          </div>
          <div class="form-control">
            <label class="label cursor-pointer justify-start gap-3 mt-8">
              <input v-model="form.accepte_nouveaux_inscrits" type="checkbox" class="toggle toggle-success" />
              <span class="label-text">Accepte les nouveaux inscrits</span>
            </label>
          </div>
        </div>

        <div v-if="mode === 'edit'" class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="form-control">
            <label class="label"><span class="label-text">Inscrits (total)</span></label>
            <input v-model.number="form.nombre_inscrits_total" type="number" min="0" class="input input-bordered">
          </div>
          <div class="form-control">
            <label class="label"><span class="label-text">Inscrits (année en cours)</span></label>
            <input v-model.number="form.nombre_inscrits_annee" type="number" min="0" class="input input-bordered">
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
